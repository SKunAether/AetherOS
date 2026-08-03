//! 恢复 Provider：操作历史与系统还原点

use aether_core::errors::CapabilityError;
use aether_core::models::cleaner::CleanerRiskLevel;
use aether_core::models::restore_point::RestorePointInfo;
use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
use aether_system::process;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::{json, Value};

use crate::history;

/// 恢复 Provider
pub struct RecoveryProvider;

impl Default for RecoveryProvider {
    fn default() -> Self {
        Self
    }
}

impl RecoveryProvider {
    /// 创建恢复 Provider
    pub fn new() -> Self {
        Self
    }

    /// 读取最近操作历史
    pub fn recent_history(
        &self,
        max: usize,
    ) -> Vec<aether_core::models::action::SystemActionRecord> {
        history::recent(max)
    }

    /// 读取系统还原点列表（PowerShell Get-ComputerRestorePoint）
    pub fn restore_points(&self) -> Result<Vec<RestorePointInfo>, CapabilityError> {
        let script = "Get-ComputerRestorePoint | Select-Object SequenceNumber,Description,CreationTime,RestorePointType | ConvertTo-Json -Compress";
        let output = process::run(
            "powershell.exe",
            &["-NoProfile", "-NonInteractive", "-Command", script],
        )?;
        if output.exit_code != 0 {
            return Ok(Vec::new()); // 系统保护可能未开启
        }
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        parse_restore_points(&stdout)
    }

    /// 创建还原点（需管理员，Checkpoint-Computer）
    pub fn create_restore_point(&self, description: &str) -> Result<i32, CapabilityError> {
        let script = format!(
            "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS'",
            description.replace('\'', "''")
        );
        process::run_elevated("powershell.exe", &["-NoProfile", "-Command", &script])
    }
}

/// 解析 Get-ComputerRestorePoint 的 JSON 输出（对象或数组）
fn parse_restore_points(stdout: &str) -> Result<Vec<RestorePointInfo>, CapabilityError> {
    if stdout.trim().is_empty() || stdout.trim() == "[]" || stdout.trim() == "null" {
        return Ok(Vec::new());
    }
    let value: Value = serde_json::from_str(stdout)
        .map_err(|e| CapabilityError::Platform(format!("parse restore points: {e}")))?;

    let mut points = Vec::new();
    let arr: Vec<Value> = match value {
        Value::Array(arr) => arr,
        Value::Object(map) => vec![Value::Object(map)],
        _ => return Ok(Vec::new()),
    };

    for item in arr {
        let sequence_number = item
            .get("SequenceNumber")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let description = item
            .get("Description")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let creation = item
            .get("CreationTime")
            .and_then(Value::as_str)
            .and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|d| d.with_timezone(&Utc))
            })
            .unwrap_or_else(Utc::now);
        let rpt = item
            .get("RestorePointType")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        points.push(RestorePointInfo {
            sequence_number,
            description,
            creation_time: creation,
            restore_point_type: rpt.to_string(),
        });
    }
    Ok(points)
}

#[async_trait]
impl CapabilityProvider for RecoveryProvider {
    fn id(&self) -> &str {
        "aether.recovery"
    }

    fn name(&self) -> &str {
        "恢复中心"
    }

    fn description(&self) -> &str {
        "操作历史、系统还原点管理"
    }

    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Recovery
    }

    fn actions(&self) -> Vec<ActionDef> {
        vec![
            ActionDef {
                id: "history".to_string(),
                name: "操作历史".to_string(),
                description: "查看最近操作记录".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
            ActionDef {
                id: "restore_points".to_string(),
                name: "还原点".to_string(),
                description: "查看系统还原点".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
        ]
    }

    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError> {
        match action {
            "history" => {
                let max = params.get("max").and_then(Value::as_u64).unwrap_or(50) as usize;
                let records = self.recent_history(max);
                serde_json::to_value(records).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "restore_points" => {
                let points = self.restore_points()?;
                serde_json::to_value(points).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "create_restore_point" => {
                let description = params
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("AetherOS")
                    .to_string();
                let exit = self.create_restore_point(&description)?;
                Ok(json!({ "exitCode": exit }))
            }
            other => Err(CapabilityError::NotImplemented(format!(
                "recovery action '{other}'"
            ))),
        }
    }

    async fn scan(&self, _params: Value) -> Result<ScanResult, CapabilityError> {
        let records = self.recent_history(20);
        Ok(ScanResult {
            provider_id: self.id().to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            data: serde_json::to_value(records)
                .map_err(|e| CapabilityError::Internal(e.to_string()))?,
            error: None,
            cancelled: false,
        })
    }
}
