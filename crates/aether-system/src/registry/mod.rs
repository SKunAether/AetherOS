//! 注册表操作封装：读写、遍历、权限处理
//!
//! 基于 windows-rs 原生调用，RAII 管理 HKEY 句柄。

use aether_core::errors::CapabilityError;
use windows::core::{HSTRING, PCWSTR, PWSTR};
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegEnumValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE, REG_DWORD, REG_MULTI_SZ,
    REG_SZ, REG_VALUE_TYPE,
};

/// 注册表根键
#[derive(Debug, Clone, Copy)]
pub enum Root {
    LocalMachine,
    CurrentUser,
}

fn root_key(root: Root) -> HKEY {
    match root {
        Root::LocalMachine => HKEY_LOCAL_MACHINE,
        Root::CurrentUser => HKEY_CURRENT_USER,
    }
}

/// 打开子键（RAII，Drop 时关闭）
struct RegKey(HKEY);

impl Drop for RegKey {
    fn drop(&mut self) {
        unsafe {
            let _ = RegCloseKey(self.0);
        }
    }
}

fn open_key(root: Root, subkey: &str) -> Result<RegKey, CapabilityError> {
    unsafe {
        let subkey_w = HSTRING::from(subkey);
        let mut key = HKEY::default();
        let err = RegOpenKeyExW(
            root_key(root),
            PCWSTR(subkey_w.as_ptr()),
            None,
            KEY_READ,
            &mut key,
        );
        if err.is_ok() {
            Ok(RegKey(key))
        } else {
            Err(CapabilityError::Platform(format!(
                "RegOpenKeyExW failed for {subkey}: code {}",
                err.0
            )))
        }
    }
}

/// 查询值原始数据（返回 (类型, 字节)）
unsafe fn query_raw(
    key: &RegKey,
    name: &str,
) -> Result<(REG_VALUE_TYPE, Vec<u8>), CapabilityError> {
    let name_w = HSTRING::from(name);
    let mut kind = REG_VALUE_TYPE(0);
    let mut buf = vec![0u8; 65536];
    let mut size = buf.len() as u32;
    let err = RegQueryValueExW(
        key.0,
        PCWSTR(name_w.as_ptr()),
        None,
        Some(&mut kind),
        Some(buf.as_mut_ptr()),
        Some(&mut size),
    );
    if err.is_err() {
        return Err(CapabilityError::NotFound(name.to_string()));
    }
    buf.truncate(size as usize);
    Ok((kind, buf))
}

/// 读取 DWORD 值
pub fn query_dword(root: Root, subkey: &str, name: &str) -> Result<u32, CapabilityError> {
    let key = open_key(root, subkey)?;
    let (kind, buf) = unsafe { query_raw(&key, name)? };
    if kind != REG_DWORD {
        return Err(CapabilityError::NotFound(format!("{subkey}\\{name}")));
    }
    if buf.len() >= 4 {
        Ok(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]))
    } else {
        Err(CapabilityError::NotFound(format!("{subkey}\\{name}")))
    }
}

/// 读取 REG_SZ 字符串值
pub fn query_string(root: Root, subkey: &str, name: &str) -> Result<String, CapabilityError> {
    let key = open_key(root, subkey)?;
    let (kind, buf) = unsafe { query_raw(&key, name)? };
    if kind != REG_SZ {
        return Err(CapabilityError::NotFound(format!("{subkey}\\{name}")));
    }
    Ok(utf16_to_string(&buf).trim_end_matches('\0').to_string())
}

/// 读取 REG_MULTI_SZ 字符串数组值
pub fn query_multi_string(
    root: Root,
    subkey: &str,
    name: &str,
) -> Result<Vec<String>, CapabilityError> {
    let key = open_key(root, subkey)?;
    let (kind, buf) = unsafe { query_raw(&key, name)? };
    if kind != REG_MULTI_SZ {
        return Err(CapabilityError::NotFound(format!("{subkey}\\{name}")));
    }
    let text = utf16_to_string(&buf);
    let mut items = Vec::new();
    for part in text.split('\0') {
        if !part.is_empty() {
            items.push(part.to_string());
        }
    }
    Ok(items)
}

/// 枚举子键下的所有值名称与数据（REG_SZ），用于启动项等场景
pub fn enumerate_string_values(
    root: Root,
    subkey: &str,
) -> Result<Vec<(String, String)>, CapabilityError> {
    unsafe {
        let key = open_key(root, subkey)?;
        let mut result = Vec::new();
        let mut index = 0u32;
        loop {
            let mut name_buf = [0u16; 512];
            let mut name_len = name_buf.len() as u32;
            let mut data_buf = [0u8; 8192];
            let mut data_len = data_buf.len() as u32;
            let mut kind = REG_VALUE_TYPE(0);
            let err = RegEnumValueW(
                key.0,
                index,
                Some(PWSTR(name_buf.as_mut_ptr())),
                &mut name_len,
                None,
                Some(&mut kind.0),
                Some(data_buf.as_mut_ptr()),
                Some(&mut data_len),
            );
            if err.is_err() {
                break;
            }
            let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
            if kind == REG_SZ || kind == REG_MULTI_SZ {
                let text = utf16_to_string(&data_buf[..data_len as usize]);
                result.push((name, text.trim_end_matches('\0').to_string()));
            }
            index += 1;
        }
        Ok(result)
    }
}

/// UTF-16LE 字节 → 字符串
fn utf16_to_string(buf: &[u8]) -> String {
    let utf16: Vec<u16> = buf
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&utf16)
}

/// 打开子键（读写权限）
fn open_key_rw(root: Root, subkey: &str) -> Result<RegKey, CapabilityError> {
    unsafe {
        let subkey_w = HSTRING::from(subkey);
        let mut key = HKEY::default();
        let err = RegOpenKeyExW(
            root_key(root),
            PCWSTR(subkey_w.as_ptr()),
            None,
            KEY_WRITE | KEY_READ,
            &mut key,
        );
        if err.is_ok() {
            Ok(RegKey(key))
        } else {
            Err(CapabilityError::Platform(format!(
                "RegOpenKeyExW (rw) failed for {subkey}: code {}",
                err.0
            )))
        }
    }
}

/// 写入 REG_SZ 字符串值
pub fn set_string(
    root: Root,
    subkey: &str,
    name: &str,
    value: &str,
) -> Result<(), CapabilityError> {
    unsafe {
        let key = open_key_rw(root, subkey)?;
        let name_w = HSTRING::from(name);
        // UTF-16 编码（含结尾 NUL）
        let mut data: Vec<u8> = value.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
        data.extend_from_slice(&[0u8, 0u8]);
        let err = RegSetValueExW(key.0, PCWSTR(name_w.as_ptr()), None, REG_SZ, Some(&data));
        if err.is_ok() {
            Ok(())
        } else {
            Err(CapabilityError::Platform(format!(
                "RegSetValueExW failed for {subkey}\\{name}: code {}",
                err.0
            )))
        }
    }
}

/// 写入 REG_MULTI_SZ 字符串数组值
pub fn set_multi_string(
    root: Root,
    subkey: &str,
    name: &str,
    values: &[String],
) -> Result<(), CapabilityError> {
    unsafe {
        let key = open_key_rw(root, subkey)?;
        let name_w = HSTRING::from(name);
        // MULTI_SZ：条目以 \0 分隔，整体以 \0\0 结尾
        let mut data: Vec<u8> = Vec::new();
        for v in values {
            data.extend(v.encode_utf16().flat_map(|u| u.to_le_bytes()));
            data.extend_from_slice(&[0u8, 0u8]);
        }
        data.extend_from_slice(&[0u8, 0u8]);
        let err = RegSetValueExW(
            key.0,
            PCWSTR(name_w.as_ptr()),
            None,
            REG_MULTI_SZ,
            Some(&data),
        );
        if err.is_ok() {
            Ok(())
        } else {
            Err(CapabilityError::Platform(format!(
                "RegSetValueExW (multi) failed for {subkey}\\{name}: code {}",
                err.0
            )))
        }
    }
}

/// 删除注册表值
pub fn delete_value(root: Root, subkey: &str, name: &str) -> Result<(), CapabilityError> {
    unsafe {
        let key = open_key_rw(root, subkey)?;
        let name_w = HSTRING::from(name);
        let err = RegDeleteValueW(key.0, PCWSTR(name_w.as_ptr()));
        if err.is_ok() {
            Ok(())
        } else {
            // 值不存在视为成功
            if err.0 == 2 {
                Ok(())
            } else {
                Err(CapabilityError::Platform(format!(
                    "RegDeleteValueW failed for {subkey}\\{name}: code {}",
                    err.0
                )))
            }
        }
    }
}
