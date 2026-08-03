//! 系统还原点模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 单个还原点
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePointInfo {
    pub sequence_number: i64,
    pub description: String,
    pub creation_time: DateTime<Utc>,
    /// APPLICATION_INSTALL / MODIFY_SETTINGS 等
    pub restore_point_type: String,
}

/// 还原点状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePointState {
    pub is_system_protection_available: bool,
    pub allocated_bytes: i64,
    pub used_bytes: i64,
    pub maximum_bytes: i64,
    pub restore_points: Vec<RestorePointInfo>,
    pub checked_at: DateTime<Utc>,
}
