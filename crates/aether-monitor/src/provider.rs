//! 监控 Provider：基于 sysinfo 实时采样 CPU/内存/磁盘

use std::sync::{Arc, Mutex};

use aether_core::errors::CapabilityError;
use aether_core::models::cleaner::CleanerRiskLevel;
use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
use aether_system::SystemEngine;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sysinfo::System;

/// 实时监控快照
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSnapshot {
    pub timestamp: chrono::DateTime<Utc>,
    pub cpu_usage_percent: f64,
    pub memory_total_bytes: u64,
    pub memory_used_bytes: u64,
    pub memory_usage_percent: f64,
    pub disk_total_bytes: u64,
    pub disk_free_bytes: u64,
    pub disk_usage_percent: f64,
    pub uptime_ms: u64,
    pub processor_count: u32,
}

/// 监控 Provider
pub struct MonitorProvider {
    system: Arc<SystemEngine>,
    sys: Mutex<System>,
}

impl MonitorProvider {
    /// 创建监控 Provider
    pub fn new(system: Arc<SystemEngine>) -> Self {
        let sys = System::new_all();
        Self {
            system,
            sys: Mutex::new(sys),
        }
    }

    /// 采集一次实时快照
    pub fn snapshot(&self) -> Result<MonitorSnapshot, CapabilityError> {
        let mut sys = self.sys.lock().unwrap();
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu = sys.global_cpu_info().cpu_usage() as f64;
        // sysinfo 0.30 内存单位：字节
        let mem_total = sys.total_memory();
        let mem_used = sys.used_memory();
        let mem_pct = if mem_total > 0 {
            (mem_used as f64 / mem_total as f64) * 100.0
        } else {
            0.0
        };

        // 系统盘容量（复用 aether-system）
        let (disk_total, disk_free) = system_disk(&self.system);

        // 核心数
        let processor_count = self
            .system
            .system_info()
            .map(|i| i.processor_count)
            .unwrap_or(0);

        let uptime_ms = self.system.system_info().map(|i| i.uptime_ms).unwrap_or(0);

        let disk_pct = if disk_total > 0 {
            ((disk_total - disk_free) as f64 / disk_total as f64) * 100.0
        } else {
            0.0
        };

        Ok(MonitorSnapshot {
            timestamp: Utc::now(),
            cpu_usage_percent: cpu,
            memory_total_bytes: mem_total,
            memory_used_bytes: mem_used,
            memory_usage_percent: mem_pct,
            disk_total_bytes: disk_total,
            disk_free_bytes: disk_free,
            disk_usage_percent: disk_pct,
            uptime_ms,
            processor_count,
        })
    }
}

/// 系统盘容量（复用 aether-system 的内部函数）
fn system_disk(_system: &Arc<SystemEngine>) -> (u64, u64) {
    // 直接调用 windows-rs GetDiskFreeSpaceExW 读取 %SYSTEMROOT% 盘
    let root = std::env::var("SYSTEMROOT").unwrap_or_else(|_| "C:\\Windows".to_string());
    let drive = root
        .chars()
        .next()
        .map(|c| format!("{c}:"))
        .unwrap_or_else(|| "C:".to_string());

    use windows::core::{HSTRING, PCWSTR};
    use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
    let path = HSTRING::from(format!("{drive}\\"));
    let mut free_available = 0u64;
    let mut total = 0u64;
    let mut total_free = 0u64;
    let result = unsafe {
        GetDiskFreeSpaceExW(
            PCWSTR(path.as_ptr()),
            Some(&mut free_available),
            Some(&mut total),
            Some(&mut total_free),
        )
    };
    if result.is_ok() {
        (total, total_free)
    } else {
        (0, 0)
    }
}

#[async_trait]
impl CapabilityProvider for MonitorProvider {
    fn id(&self) -> &str {
        "aether.monitor"
    }

    fn name(&self) -> &str {
        "系统监控"
    }

    fn description(&self) -> &str {
        "CPU / 内存 / 磁盘实时监控与系统信息采集"
    }

    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Monitor
    }

    fn actions(&self) -> Vec<ActionDef> {
        vec![ActionDef {
            id: "snapshot".to_string(),
            name: "采集快照".to_string(),
            description: "采集当前 CPU/内存/磁盘实时数据".to_string(),
            risk_level: CleanerRiskLevel::Low,
            requires_administrator: false,
            is_reversible: false,
        }]
    }

    async fn execute(&self, action: &str, _params: Value) -> Result<Value, CapabilityError> {
        match action {
            "snapshot" => {
                let snap = self.snapshot()?;
                serde_json::to_value(snap).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            other => Err(CapabilityError::NotImplemented(format!(
                "monitor action '{other}'"
            ))),
        }
    }

    async fn scan(&self, _params: Value) -> Result<ScanResult, CapabilityError> {
        let snap = self.snapshot()?;
        Ok(ScanResult {
            provider_id: self.id().to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            data: serde_json::to_value(snap)
                .map_err(|e| CapabilityError::Internal(e.to_string()))?,
            error: None,
            cancelled: false,
        })
    }
}

/// 便捷：构造监控 Provider（供 src-tauri 注册）
pub fn register_monitor(
    registry: &aether_runtime::registry::ProviderRegistry,
    system: Arc<SystemEngine>,
) {
    registry.register(MonitorProvider::new(system));
    let _ = json!({});
}
