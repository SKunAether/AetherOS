//! 休眠状态模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 休眠功能状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HibernateState {
    pub is_enabled: bool,
    pub is_hibernate_file_present: bool,
    pub hibernate_file_bytes: i64,
    pub system_drive: String,
    pub checked_at: DateTime<Utc>,
}
