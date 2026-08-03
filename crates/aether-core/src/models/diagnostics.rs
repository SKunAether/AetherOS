//! 诊断模型：健康检查、健康评分

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 诊断项严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DiagnosticSeverity {
    Information,
    Success,
    Warning,
    Error,
}

/// 单个诊断检查项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticCheckResult {
    pub category: String,
    pub name: String,
    pub severity: DiagnosticSeverity,
    pub summary: String,
    pub details: String,
    pub recommendation: String,
}

/// 诊断快照（健康分服务端计算：100 - Error*25 - Warning*8）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticSnapshot {
    pub created_at: DateTime<Utc>,
    pub machine_name: String,
    pub operating_system: String,
    pub framework: String,
    pub is_administrator: bool,
    pub system_drive_free_bytes: i64,
    pub checks: Vec<DiagnosticCheckResult>,
    pub error_count: i64,
    pub warning_count: i64,
    pub success_count: i64,
    pub health_score: i64,
    pub summary: String,
}
