//! 优化 Provider：电源计划、休眠、启动项

use std::path::PathBuf;

use aether_core::errors::CapabilityError;
use aether_core::models::action::SystemActionRecord;
use aether_core::models::cleaner::CleanerRiskLevel;
use aether_core::models::hibernate::HibernateState;
use aether_core::models::pagefile::PageFileConfiguration;
use aether_core::models::power::{PowerPlanInfo, PowerPlanState};
use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
use aether_system::registry::{query_dword, Root};
use aether_system::SystemEngine;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::startup;

/// 优化 Provider
pub struct OptimizerProvider {
    system: std::sync::Arc<SystemEngine>,
}

impl OptimizerProvider {
    /// 创建优化 Provider
    pub fn new(system: std::sync::Arc<SystemEngine>) -> Self {
        Self { system }
    }

    /// 读取电源计划列表
    pub fn power_plans(&self) -> Result<PowerPlanState, CapabilityError> {
        let plans = self.system.power_plans()?;
        let list: Vec<PowerPlanInfo> = plans
            .into_iter()
            .map(|p| PowerPlanInfo {
                id: p.id.parse().unwrap_or_else(|_| Uuid::nil()),
                name: p.name,
                is_active: p.is_active,
                is_built_in: p.category != "Custom",
                category: p.category,
            })
            .collect();
        let active_plan_id = list.iter().find(|p| p.is_active).map(|p| p.id);
        Ok(PowerPlanState {
            plans: list,
            active_plan_id,
            checked_at: Utc::now(),
        })
    }

    /// 激活电源计划（需管理员）
    pub fn activate_power_plan(
        &self,
        plan_id: Uuid,
    ) -> Result<SystemActionRecord, CapabilityError> {
        let before = self
            .power_plans()?
            .active_plan_id
            .map(|i| i.to_string())
            .unwrap_or_default();
        let exit_code = self.system.activate_power_plan(plan_id)?;
        let record = SystemActionRecord {
            id: Uuid::new_v4(),
            rule_id: "optimizer.activate-power-plan".to_string(),
            display_name: "切换电源计划".to_string(),
            action: format!("activate plan {plan_id}"),
            previous_state: before,
            result_state: plan_id.to_string(),
            succeeded: exit_code == 0,
            exit_code,
            error_message: if exit_code == 0 {
                None
            } else {
                Some(format!("exit {exit_code}"))
            },
            executed_at: Utc::now(),
        };
        self::startup_ops::record_action(&record);
        Ok(record)
    }

    /// 读取休眠状态
    pub fn hibernate_state(&self) -> Result<HibernateState, CapabilityError> {
        let info = self.system.system_info()?;
        let system_drive = info.system_drive.clone();

        // 注册表 HibernateEnabled
        let reg_enabled = query_dword(
            Root::LocalMachine,
            "SYSTEM\\CurrentControlSet\\Control\\Power",
            "HibernateEnabled",
        )
        .map(|v| v != 0)
        .unwrap_or(false);

        // hiberfil.sys 存在性与大小
        let hibernate_file = PathBuf::from(format!("{system_drive}\\hiberfil.sys"));
        let (file_present, file_bytes) = std::fs::metadata(&hibernate_file)
            .map(|m| (true, m.len() as i64))
            .unwrap_or((false, 0));

        Ok(HibernateState {
            is_enabled: reg_enabled || file_present,
            is_hibernate_file_present: file_present,
            hibernate_file_bytes: file_bytes,
            system_drive,
            checked_at: Utc::now(),
        })
    }

    /// 开关休眠（需管理员）
    pub fn set_hibernate(&self, enabled: bool) -> Result<SystemActionRecord, CapabilityError> {
        let before = self.hibernate_state()?.is_enabled;
        let exit_code = self.system.set_hibernate(enabled)?;
        let record = SystemActionRecord {
            id: Uuid::new_v4(),
            rule_id: "optimizer.set-hibernate".to_string(),
            display_name: if enabled {
                "启用休眠".to_string()
            } else {
                "禁用休眠".to_string()
            },
            action: format!("hibernate {enabled}"),
            previous_state: before.to_string(),
            result_state: enabled.to_string(),
            succeeded: exit_code == 0,
            exit_code,
            error_message: if exit_code == 0 {
                None
            } else {
                Some(format!("exit {exit_code}"))
            },
            executed_at: Utc::now(),
        };
        self::startup_ops::record_action(&record);
        Ok(record)
    }

    /// 读取启动项列表
    pub fn startup_items(&self) -> Vec<startup::StartupItem> {
        startup::list_startup_items()
    }

    /// 读取虚拟内存配置（对齐 C# WindowsPageFileService / VirtualMemoryService）
    pub fn virtual_memory(&self) -> Result<PageFileConfiguration, CapabilityError> {
        use aether_core::models::pagefile::{PageFileConfiguration, PageFileEntry};
        use aether_system::registry::{query_multi_string, Root};

        let reg_entries = query_multi_string(
            Root::LocalMachine,
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
            "PagingFiles",
        )
        .unwrap_or_default();

        // 自动管理：PagingFiles 无条目或值为空
        let is_automatic = reg_entries.is_empty();

        let mut entries = Vec::new();
        for raw in &reg_entries {
            // 格式："C:\pagefile.sys 1024 2048"
            let parts: Vec<&str> = raw.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let path = parts[0].to_string();
            let initial = parts
                .get(1)
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0);
            let maximum = parts
                .get(2)
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0);
            let (exists, current_bytes) = std::fs::metadata(&path)
                .map(|m| (true, m.len() as i64))
                .unwrap_or((false, 0));
            entries.push(PageFileEntry {
                path,
                initial_size_mb: initial,
                maximum_size_mb: maximum,
                current_file_bytes: current_bytes,
                exists,
            });
        }

        let total_bytes = entries.iter().map(|e| e.current_file_bytes).sum();
        Ok(PageFileConfiguration {
            is_automatically_managed: is_automatic,
            entries,
            total_current_file_bytes: total_bytes,
            restart_required: false,
            checked_at: Utc::now(),
        })
    }

    /// 应用虚拟内存配置（写注册表 PagingFiles，需管理员 + 重启生效）
    pub fn apply_virtual_memory(
        &self,
        automatic: bool,
        entries: Vec<String>,
    ) -> Result<SystemActionRecord, CapabilityError> {
        use aether_system::registry::{set_multi_string, Root};

        let subkey = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management";
        let before = self.virtual_memory()?.is_automatically_managed;
        // 自动管理 → 清空 PagingFiles
        let values = if automatic { vec![] } else { entries };
        set_multi_string(Root::LocalMachine, subkey, "PagingFiles", &values)?;

        let record = SystemActionRecord {
            id: Uuid::new_v4(),
            rule_id: "optimizer.apply-virtual-memory".to_string(),
            display_name: "修改虚拟内存".to_string(),
            action: format!("virtual memory automatic={automatic}"),
            previous_state: before.to_string(),
            result_state: automatic.to_string(),
            succeeded: true,
            exit_code: 0,
            error_message: None,
            executed_at: Utc::now(),
        };
        self::startup_ops::record_action(&record);
        Ok(record)
    }
}

/// 操作历史记录辅助（写入 %LOCALAPPDATA%\AetherOS\History\）
pub mod startup_ops {
    use aether_core::models::action::SystemActionRecord;
    use std::path::PathBuf;

    /// 记录一条操作审计记录
    pub fn record_action(record: &SystemActionRecord) {
        let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
        let dir = PathBuf::from(base).join("AetherOS").join("History");
        let _ = std::fs::create_dir_all(&dir);
        let stamp = record.executed_at.format("%Y%m%d-%H%M%S");
        let file = dir.join(format!("{stamp}-{}.json", record.id));
        if let Ok(json) = serde_json::to_string_pretty(record) {
            let _ = std::fs::write(file, json);
        }
    }
}

#[async_trait]
impl CapabilityProvider for OptimizerProvider {
    fn id(&self) -> &str {
        "aether.optimizer"
    }

    fn name(&self) -> &str {
        "性能优化"
    }

    fn description(&self) -> &str {
        "电源计划、休眠、启动项管理"
    }

    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Optimizer
    }

    fn actions(&self) -> Vec<ActionDef> {
        vec![
            ActionDef {
                id: "get_power_plans".to_string(),
                name: "电源计划".to_string(),
                description: "查看电源计划列表".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
            ActionDef {
                id: "get_hibernate_state".to_string(),
                name: "休眠状态".to_string(),
                description: "查看休眠状态".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
            ActionDef {
                id: "get_startup_items".to_string(),
                name: "启动项".to_string(),
                description: "查看启动项列表".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
            ActionDef {
                id: "get_virtual_memory".to_string(),
                name: "虚拟内存".to_string(),
                description: "查看虚拟内存配置".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
            ActionDef {
                id: "apply_virtual_memory".to_string(),
                name: "修改虚拟内存".to_string(),
                description: "应用虚拟内存配置（需管理员）".to_string(),
                risk_level: CleanerRiskLevel::High,
                requires_administrator: true,
                is_reversible: true,
            },
        ]
    }

    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError> {
        match action {
            "get_power_plans" => {
                let state = self.power_plans()?;
                serde_json::to_value(state).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "activate_power_plan" => {
                let plan_id: Uuid = params
                    .get("planId")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(|| CapabilityError::InvalidArgument("planId".to_string()))?;
                let record = self.activate_power_plan(plan_id)?;
                serde_json::to_value(record).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "get_hibernate_state" => {
                let state = self.hibernate_state()?;
                serde_json::to_value(state).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "set_hibernate" => {
                let enabled = params
                    .get("enabled")
                    .and_then(Value::as_bool)
                    .ok_or_else(|| CapabilityError::InvalidArgument("enabled".to_string()))?;
                let record = self.set_hibernate(enabled)?;
                serde_json::to_value(record).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "get_startup_items" => {
                let items = self.startup_items();
                serde_json::to_value(items).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "set_startup_item" => {
                let name = params
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| CapabilityError::InvalidArgument("name".to_string()))?;
                let command = params
                    .get("command")
                    .and_then(Value::as_str)
                    .ok_or_else(|| CapabilityError::InvalidArgument("command".to_string()))?;
                startup::set_startup_item(name, command)?;
                Ok(json!({ "ok": true }))
            }
            "delete_startup_item" => {
                let name = params
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| CapabilityError::InvalidArgument("name".to_string()))?;
                let scope = params
                    .get("scope")
                    .and_then(Value::as_str)
                    .unwrap_or("HKCU");
                startup::delete_startup_item(name, scope)?;
                Ok(json!({ "ok": true }))
            }
            "get_virtual_memory" => {
                let config = self.virtual_memory()?;
                serde_json::to_value(config).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "apply_virtual_memory" => {
                let automatic = params
                    .get("automatic")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let entries: Vec<String> = params
                    .get("entries")
                    .and_then(Value::as_array)
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let record = self.apply_virtual_memory(automatic, entries)?;
                serde_json::to_value(record).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            other => Err(CapabilityError::NotImplemented(format!(
                "optimizer action '{other}'"
            ))),
        }
    }

    async fn scan(&self, _params: Value) -> Result<ScanResult, CapabilityError> {
        let plans = self.power_plans()?;
        let hibernate = self.hibernate_state()?;
        Ok(ScanResult {
            provider_id: self.id().to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            data: json!({ "powerPlans": plans, "hibernate": hibernate }),
            error: None,
            cancelled: false,
        })
    }
}
