//! 操作记录模型（对齐 C# OperationRecord / ConfigurationChangeRecord）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 操作记录（JSONL 追加日志）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub category: String,
    pub action: String,
    pub result: String,
    pub details: String,
    pub plan_id: Option<Uuid>,
    pub action_id: Option<Uuid>,
}

/// 配置变更记录（用于回滚）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationChangeRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub category: String,
    pub action: String,
    pub before_value: String,
    pub after_value: String,
    pub is_reversible: bool,
    pub is_restored: bool,
    pub restore_payload: String,
}
