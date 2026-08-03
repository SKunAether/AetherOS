//! 电源计划模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 单个电源计划
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerPlanInfo {
    /// 电源计划 GUID
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub is_built_in: bool,
    /// Balanced / HighPerformance / PowerSaver / Custom
    pub category: String,
}

/// 电源计划状态快照
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerPlanState {
    pub plans: Vec<PowerPlanInfo>,
    pub active_plan_id: Option<Uuid>,
    pub checked_at: DateTime<Utc>,
}
