//! 开机自启管理：注册表 Run 键写入/移除
//!
//! 对齐旧版 C# `StartupManager`：HKCU\Software\Microsoft\Windows\CurrentVersion\Run

use aether_system::registry::{delete_value, set_string, Root};

const RUN_SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "AetherOS Guardian";

/// 注册开机自启（写入 Run 键）
pub fn enable(exe_path: &str) -> Result<(), String> {
    let value = format!("\"{exe_path}\" --tray");
    set_string(Root::CurrentUser, RUN_SUBKEY, RUN_VALUE, &value).map_err(|e| e.to_string())
}

/// 移除开机自启
pub fn disable() -> Result<(), String> {
    delete_value(Root::CurrentUser, RUN_SUBKEY, RUN_VALUE).map_err(|e| e.to_string())
}

/// 根据设置应用开机自启状态
pub fn apply(exe_path: &str, run_at_startup: bool) -> Result<(), String> {
    if run_at_startup {
        enable(exe_path)
    } else {
        disable()
    }
}
