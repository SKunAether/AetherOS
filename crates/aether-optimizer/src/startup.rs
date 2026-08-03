//! 启动项管理：读取/写入注册表 Run 键
//!
//! 对齐旧版 C# `StartupManager`：HKLM + HKCU 的 Run 键，
//! 每个启动项是独立的 REG_SZ 值（值名=应用名，值数据=命令）。

use aether_core::errors::CapabilityError;
use aether_system::registry::{delete_value, enumerate_string_values, set_string, Root};
use serde::{Deserialize, Serialize};

/// 启动项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupItem {
    pub name: String,
    pub command: String,
    /// HKLM 或 HKCU
    pub scope: String,
    pub is_enabled: bool,
}

const RUN_SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";

/// 读取所有启动项（HKLM + HKCU 合并，HKCU 优先去重）
pub fn list_startup_items() -> Vec<StartupItem> {
    let mut items = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (scope, root) in [("HKLM", Root::LocalMachine), ("HKCU", Root::CurrentUser)] {
        if let Ok(values) = enumerate_string_values(root, RUN_SUBKEY) {
            for (name, command) in values {
                if name.is_empty() || command.is_empty() {
                    continue;
                }
                let key = name.to_lowercase();
                // HKCU 覆盖 HKLM 的同名项
                if scope == "HKLM" && seen.contains(&key) {
                    continue;
                }
                seen.insert(key);
                items.push(StartupItem {
                    name,
                    command,
                    scope: scope.to_string(),
                    is_enabled: true,
                });
            }
        }
    }
    items
}

/// 新增/更新启动项（写入 HKCU Run 键）
pub fn set_startup_item(name: &str, command: &str) -> Result<(), CapabilityError> {
    set_string(Root::CurrentUser, RUN_SUBKEY, name, command)
}

/// 移除启动项（HKCU 优先，其次 HKLM）
pub fn delete_startup_item(name: &str, scope: &str) -> Result<(), CapabilityError> {
    let root = match scope {
        "HKLM" => Root::LocalMachine,
        _ => Root::CurrentUser,
    };
    delete_value(root, RUN_SUBKEY, name)
}
