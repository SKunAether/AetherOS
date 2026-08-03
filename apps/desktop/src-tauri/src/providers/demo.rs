//! 演示 Provider（aether.demo）：验证 CapabilityProvider 全链路
//!
//! 提供 ping 动作与一个最小 scan，用于 Phase 1 打通
//! 前端 → IPC → 运行时 → Provider → 事件总线的完整调用链。

use std::sync::Arc;

use aether_core::errors::CapabilityError;
use aether_core::models::cleaner::CleanerRiskLevel;
use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
use aether_system::SystemEngine;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::{json, Value};

/// 演示 Provider
pub struct DemoProvider {
    system: Arc<SystemEngine>,
}

impl DemoProvider {
    /// 创建演示 Provider
    pub fn new() -> Self {
        Self {
            system: SystemEngine::new(),
        }
    }
}

impl Default for DemoProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CapabilityProvider for DemoProvider {
    fn id(&self) -> &str {
        "aether.demo"
    }

    fn name(&self) -> &str {
        "Demo Provider"
    }

    fn description(&self) -> &str {
        "验证 CapabilityProvider 全链路的演示能力提供者"
    }

    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Extension
    }

    fn actions(&self) -> Vec<ActionDef> {
        vec![ActionDef {
            id: "ping".to_string(),
            name: "Ping".to_string(),
            description: "返回 pong，验证 IPC 链路".to_string(),
            risk_level: CleanerRiskLevel::Low,
            requires_administrator: false,
            is_reversible: false,
        }]
    }

    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError> {
        match action {
            "ping" => {
                let message = params
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("pong");
                Ok(json!({ "result": message, "echo": true }))
            }
            "system-info" => {
                let info = self.system.system_info()?;
                serde_json::to_value(info).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            other => Err(CapabilityError::NotImplemented(format!(
                "demo action '{other}'"
            ))),
        }
    }

    async fn scan(&self, _params: Value) -> Result<ScanResult, CapabilityError> {
        let info = self.system.system_info()?;
        Ok(ScanResult {
            provider_id: self.id().to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            data: serde_json::to_value(info)
                .map_err(|e| CapabilityError::Internal(e.to_string()))?,
            error: None,
            cancelled: false,
        })
    }
}
