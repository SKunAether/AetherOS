//! 系统操作审计记录（对齐 C# SystemActionRecord）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 统一的操作审计记录：每次系统变更（清理/优化/恢复）落一条
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemActionRecord {
    pub id: Uuid,
    /// 触发规则 ID（如 "cleaner.execute-approved-plan"）
    pub rule_id: String,
    pub display_name: String,
    /// 执行的动作描述
    pub action: String,
    pub previous_state: String,
    pub result_state: String,
    pub succeeded: bool,
    pub exit_code: i32,
    pub error_message: Option<String>,
    pub executed_at: DateTime<Utc>,
}
