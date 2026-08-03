//! 清理能力领域模型：扫描项、扫描结果、执行结果

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 清理项风险等级（沿用 C# PascalCase 枚举值，兼容旧版记录）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CleanerRiskLevel {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

/// 清理扫描项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanerItem {
    /// 规则 ID，如 "cleaner.user-temp"
    pub id: String,
    pub name: String,
    pub description: String,
    /// 目标路径
    pub path: String,
    /// 估算可释放字节数
    #[serde(default)]
    pub estimated_bytes: i64,
    #[serde(default)]
    pub risk_level: CleanerRiskLevel,
    #[serde(default)]
    pub requires_administrator: bool,
    #[serde(default)]
    pub is_selected_by_default: bool,
}

/// 清理扫描结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanerScanResult {
    #[serde(default)]
    pub items: Vec<CleanerItem>,
    /// 总估算可释放字节数（服务端计算）
    #[serde(default)]
    pub total_estimated_bytes: i64,
    pub completed_at: DateTime<Utc>,
}

/// 单个清理项执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanerExecutionItemResult {
    pub item_id: String,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub before_bytes: i64,
    #[serde(default)]
    pub after_bytes: i64,
    /// 已释放字节数（服务端计算 = before - after）
    #[serde(default)]
    pub released_bytes: i64,
    #[serde(default)]
    pub deleted_file_count: i64,
    #[serde(default)]
    pub skipped_file_count: i64,
    #[serde(default)]
    pub succeeded: bool,
    #[serde(default)]
    pub requires_administrator: bool,
    #[serde(default)]
    pub error_message: Option<String>,
}

/// 清理执行总结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanerExecutionResult {
    #[serde(default)]
    pub items: Vec<CleanerExecutionItemResult>,
    #[serde(default)]
    pub total_released_bytes: i64,
    #[serde(default)]
    pub total_deleted_file_count: i64,
    #[serde(default)]
    pub total_skipped_file_count: i64,
    /// 全部成功才算成功
    #[serde(default)]
    pub succeeded: bool,
    pub completed_at: DateTime<Utc>,
}
