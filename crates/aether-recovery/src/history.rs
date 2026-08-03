//! 操作历史服务：JSON 文件持久化与读取
//!
//! 对齐旧版 C# `JsonActionHistoryService`：记录写入
//! `%LOCALAPPDATA%\AetherOS\History\{timestamp}-{guid}.json`。

use std::path::PathBuf;

use aether_core::errors::CapabilityError;
use aether_core::models::action::SystemActionRecord;

/// 历史目录
pub fn history_dir() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(base).join("AetherOS").join("History")
}

/// 追加一条操作记录
pub fn record_action(record: &SystemActionRecord) -> Result<(), CapabilityError> {
    let dir = history_dir();
    std::fs::create_dir_all(&dir)?;
    let stamp = record.executed_at.format("%Y%m%d-%H%M%S");
    let file = dir.join(format!("{stamp}-{}.json", record.id));
    let json = serde_json::to_string_pretty(record)
        .map_err(|e| CapabilityError::Internal(e.to_string()))?;
    std::fs::write(file, json)?;
    Ok(())
}

/// 读取最近的操作记录（按时间倒序）
pub fn recent(max: usize) -> Vec<SystemActionRecord> {
    let dir = history_dir();
    let mut records: Vec<(String, SystemActionRecord)> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    if let Ok(record) = serde_json::from_str::<SystemActionRecord>(&text) {
                        let key = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        records.push((key, record));
                    }
                }
            }
        }
    }
    records.sort_by(|a, b| b.0.cmp(&a.0));
    records.into_iter().take(max).map(|(_, r)| r).collect()
}

/// 打开历史目录
pub fn open_history_dir() {
    let dir = history_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::process::Command::new("explorer.exe").arg(&dir).spawn();
}
