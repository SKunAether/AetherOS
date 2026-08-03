//! 系统上下文快照：仪表盘与 AI 分析的基础数据

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 系统上下文快照
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemContextSnapshot {
    pub captured_at: DateTime<Utc>,
    pub machine_name: String,
    pub operating_system: String,
    pub processor_count: i64,
    pub system_uptime: String,
    pub system_drive: String,
    pub total_drive_bytes: i64,
    pub free_drive_bytes: i64,
    pub used_drive_percentage: i64,
    pub temporary_files_bytes: i64,
    pub windows_update_cache_bytes: i64,
    pub hibernate_file_bytes: i64,
    pub page_file_bytes: i64,
    pub windows_old_bytes: i64,
    pub is_administrator: bool,
    /// 健康评分（服务端计算）
    pub health_score: i64,
    /// 关注项数量
    pub attention_count: i64,
    /// 置信度（管理员 100，否则 85）
    pub confidence_score: i64,
    pub confidence_label: String,
    pub findings: Vec<String>,
}
