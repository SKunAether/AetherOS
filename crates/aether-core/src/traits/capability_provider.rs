//! CapabilityProvider 核心抽象：所有系统能力的统一接口
//!
//! 设计要点：
//! - object-safe：注册中心以 `Arc<dyn CapabilityProvider>` 存储，不能用关联类型/泛型返回；
//! - JSON 边界：参数与结果均为 `serde_json::Value`，是插件边界（aether-plugin-sdk）的稳定线上格式；
//! - scan 接受 params：支持"扫描哪个目录 / 哪种快照"等带参扫描。

use crate::errors::CapabilityError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 能力类型：划分 Provider 所属的功能域
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityType {
    /// 系统清理
    Cleaner,
    /// 系统优化
    Optimizer,
    /// 状态监控
    Monitor,
    /// 备份恢复
    Recovery,
    /// 其他/扩展能力（第三方插件）
    Extension,
}

/// 动作定义：声明一个 Provider 可执行的动作及其风险属性
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDef {
    /// 动作标识，如 "scan" / "execute" / "get_plans"
    pub id: String,
    /// 动作显示名称
    pub name: String,
    /// 动作说明
    #[serde(default)]
    pub description: String,
    /// 风险等级
    #[serde(default)]
    pub risk_level: crate::models::cleaner::CleanerRiskLevel,
    /// 是否需要管理员权限
    #[serde(default)]
    pub requires_administrator: bool,
    /// 是否可回滚
    #[serde(default)]
    pub is_reversible: bool,
}

/// 扫描结果：Provider 统一上报的扫描产出
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    /// Provider 标识
    pub provider_id: String,
    /// 扫描开始时间
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// 扫描完成时间
    pub completed_at: chrono::DateTime<chrono::Utc>,
    /// 扫描产出数据（JSON，由各 Provider 定义自己的 schema）
    #[serde(default)]
    pub data: Value,
    /// 扫描错误信息（若有）
    #[serde(default)]
    pub error: Option<String>,
    /// 是否被取消
    #[serde(default)]
    pub cancelled: bool,
}

/// Capability Provider 通用抽象
#[async_trait]
pub trait CapabilityProvider: Send + Sync {
    /// Provider 唯一标识
    fn id(&self) -> &str;

    /// Provider 显示名称
    fn name(&self) -> &str;

    /// Provider 描述（可选）
    fn description(&self) -> &str {
        ""
    }

    /// 能力类型
    fn capability_type(&self) -> CapabilityType;

    /// 声明的可执行动作列表（可为空）
    fn actions(&self) -> Vec<ActionDef> {
        Vec::new()
    }

    /// 执行指定动作（返回 JSON 结果）
    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError>;

    /// 触发一次能力扫描（快照 / 清理扫描 / 监控采样）
    async fn scan(&self, params: Value) -> Result<ScanResult, CapabilityError>;
}
