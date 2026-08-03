//! 快速扫描结果模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 快速扫描结果：系统盘与常见可回收空间快照
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickScanResult {
    pub system_drive_name: String,
    pub total_drive_bytes: i64,
    pub free_drive_bytes: i64,
    /// 已用字节数（服务端计算）
    pub used_drive_bytes: i64,
    /// 已用百分比（服务端计算）
    pub used_drive_percentage: i64,
    pub is_administrator: bool,
    pub is_hibernate_file_present: bool,
    pub hibernate_file_bytes: i64,
    pub is_page_file_present: bool,
    pub page_file_bytes: i64,
    pub temporary_files_bytes: i64,
    pub windows_update_cache_bytes: i64,
    pub is_windows_old_present: bool,
    pub windows_old_bytes: i64,
    pub completed_at: DateTime<Utc>,
}
