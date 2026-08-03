//! 硬件信息读取：CPU、内存、磁盘、系统信息
//!
//! 基于 windows-rs 原生调用，无 WMI/外部进程依赖。

use aether_core::errors::CapabilityError;
use serde::{Deserialize, Serialize};

/// 系统信息快照
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub machine_name: String,
    /// 如 "Windows 11 24H2 (Build 26100)"
    pub os_version: String,
    pub os_build: u32,
    pub processor_count: u32,
    pub physical_memory_bytes: u64,
    pub available_memory_bytes: u64,
    /// 系统盘盘符，如 "C:"
    pub system_drive: String,
    pub system_drive_total_bytes: u64,
    pub system_drive_free_bytes: u64,
    /// 系统启动时长（毫秒）
    pub uptime_ms: u64,
    pub is_administrator: bool,
}

/// 采集系统信息
pub fn collect() -> Result<SystemInfo, CapabilityError> {
    unsafe {
        let machine_name = get_computer_name()?;
        let (os_version, os_build) = get_os_version()?;
        let processor_count = get_processor_count();
        let (physical_memory, available_memory) = get_memory_status()?;
        let system_drive = get_system_drive()?;
        let (total_bytes, free_bytes) = get_disk_free_space(&system_drive)?;
        let uptime_ms = get_tick_count64();
        let is_administrator = is_elevated()?;

        Ok(SystemInfo {
            machine_name,
            os_version,
            os_build,
            processor_count,
            physical_memory_bytes: physical_memory,
            available_memory_bytes: available_memory,
            system_drive,
            system_drive_total_bytes: total_bytes,
            system_drive_free_bytes: free_bytes,
            uptime_ms,
            is_administrator,
        })
    }
}

/// 获取计算机名（GetComputerNameW 位于 WindowsProgramming 模块）
unsafe fn get_computer_name() -> Result<String, CapabilityError> {
    use windows::core::PWSTR;
    use windows::Win32::System::WindowsProgramming::GetComputerNameW;

    let mut buf = [0u16; 64];
    let mut size = buf.len() as u32;
    match GetComputerNameW(Some(PWSTR(buf.as_mut_ptr())), &mut size) {
        Ok(()) => Ok(String::from_utf16_lossy(&buf[..size as usize])),
        Err(_) => Ok("Unknown".to_string()),
    }
}

/// 获取 OS 版本（GetVersionExW）
unsafe fn get_os_version() -> Result<(String, u32), CapabilityError> {
    use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};

    let mut info = std::mem::zeroed::<OSVERSIONINFOW>();
    info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;
    if GetVersionExW(&mut info).is_ok() {
        let major = info.dwMajorVersion;
        let build = info.dwBuildNumber;
        let product = if major >= 10 && build >= 22000 {
            "Windows 11"
        } else {
            "Windows 10"
        };
        Ok((format!("{product} (Build {build})"), build))
    } else {
        Ok(("Unknown".to_string(), 0))
    }
}

/// CPU 逻辑核心数
unsafe fn get_processor_count() -> u32 {
    use windows::Win32::System::SystemInformation::{GetNativeSystemInfo, SYSTEM_INFO};
    let mut info = std::mem::zeroed::<SYSTEM_INFO>();
    GetNativeSystemInfo(&mut info);
    info.dwNumberOfProcessors
}

/// 物理内存与可用内存（GlobalMemoryStatusEx 位于 SystemInformation 模块）
unsafe fn get_memory_status() -> Result<(u64, u64), CapabilityError> {
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

    let mut status = std::mem::zeroed::<MEMORYSTATUSEX>();
    status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    match GlobalMemoryStatusEx(&mut status) {
        Ok(()) => Ok((status.ullTotalPhys, status.ullAvailPhys)),
        Err(e) => Err(CapabilityError::Platform(format!(
            "GlobalMemoryStatusEx failed: {e}"
        ))),
    }
}

/// 系统盘盘符（取 %SYSTEMROOT% 首字符）
unsafe fn get_system_drive() -> Result<String, CapabilityError> {
    let root = std::env::var("SYSTEMROOT").unwrap_or_else(|_| "C:\\Windows".to_string());
    Ok(root
        .chars()
        .next()
        .map(|c| format!("{c}:"))
        .unwrap_or_else(|| "C:".to_string()))
}

/// 磁盘总容量与可用容量（字节）
unsafe fn get_disk_free_space(drive: &str) -> Result<(u64, u64), CapabilityError> {
    use windows::core::{HSTRING, PCWSTR};
    use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

    let path = HSTRING::from(format!("{drive}\\"));
    let mut free_available = 0u64;
    let mut total = 0u64;
    let mut total_free = 0u64;
    match GetDiskFreeSpaceExW(
        PCWSTR(path.as_ptr()),
        Some(&mut free_available),
        Some(&mut total),
        Some(&mut total_free),
    ) {
        Ok(()) => Ok((total, total_free)),
        Err(e) => Err(CapabilityError::Platform(format!(
            "GetDiskFreeSpaceExW failed for {drive}: {e}"
        ))),
    }
}

/// 系统启动时长（毫秒）
unsafe fn get_tick_count64() -> u64 {
    windows::Win32::System::SystemInformation::GetTickCount64()
}

/// 当前进程是否管理员（TokenElevation 判定，令牌 API 位于 Security 模块）
unsafe fn is_elevated() -> Result<bool, CapabilityError> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    let process = GetCurrentProcess();
    let mut token = std::mem::zeroed::<HANDLE>();
    if OpenProcessToken(process, TOKEN_QUERY, &mut token).is_err() {
        return Ok(false);
    }

    let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let mut return_len = 0u32;
    match GetTokenInformation(
        token,
        TokenElevation,
        Some(&mut elevation as *mut _ as *mut std::ffi::c_void),
        std::mem::size_of::<TOKEN_ELEVATION>() as u32,
        &mut return_len,
    ) {
        Ok(()) => Ok(elevation.TokenIsElevated != 0),
        Err(_) => Ok(false),
    }
}
