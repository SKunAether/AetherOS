//! 维护/优化执行模型（对齐 C# Maintenance* 系列）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 维护风险等级
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaintenanceRiskLevel {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

/// 审批状态
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaintenanceApprovalStatus {
    #[default]
    Draft,
    Approved,
    Rejected,
    Executed,
    Failed,
    RolledBack,
}

/// 执行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaintenanceExecutionStatus {
    Pending,
    Skipped,
    Running,
    Succeeded,
    Failed,
    VerificationFailed,
    Blocked,
}

/// 单个维护动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceAction {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    /// 功能域分类（DiskSpace / Hibernate / Cleanup / SystemConfig / Startup / Recovery）
    pub category: String,
    #[serde(default)]
    pub risk_level: MaintenanceRiskLevel,
    #[serde(default)]
    pub status: MaintenanceApprovalStatus,
    #[serde(default)]
    pub is_selected: bool,
    #[serde(default)]
    pub requires_administrator: bool,
    #[serde(default)]
    pub requires_restart: bool,
    #[serde(default)]
    pub is_reversible: bool,
    /// 执行后验证描述
    pub verification: String,
    pub rollback_instruction: String,
}

/// 维护计划（一组动作）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenancePlan {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub rule_set_name: String,
    pub source_summary: String,
    /// 证据快照（系统上下文 JSON）
    pub evidence_snapshot: serde_json::Value,
    pub status: MaintenanceApprovalStatus,
    pub actions: Vec<MaintenanceAction>,
}

/// 单个动作执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceExecutionResult {
    pub action_id: Uuid,
    pub action_title: String,
    /// 执行器（HibernateService / PowerPlanService 等）
    pub executor: String,
    pub status: MaintenanceExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub before_value: String,
    pub after_value: String,
    pub message: String,
    #[serde(default)]
    pub verification_passed: bool,
    #[serde(default)]
    pub can_rollback: bool,
    /// 回滚数据（序列化字符串，由 Executor 解释）
    #[serde(default)]
    pub rollback_data: String,
    #[serde(default)]
    pub rollback_completed: bool,
    pub rolled_back_at: Option<DateTime<Utc>>,
    pub rollback_message: String,
}

/// 维护计划执行报告
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenancePlanExecutionReport {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub plan_title: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub restore_point_requested: bool,
    #[serde(default)]
    pub restore_point_created: bool,
    pub restore_point_message: String,
    pub results: Vec<MaintenanceExecutionResult>,
    /// 全部成功才算成功
    pub succeeded: bool,
    pub summary: String,
}
