//! 任务调度器：异步任务编排、并行执行、进度与超时管理
//!
//! Phase 1 实现"立即返回 task_id + 事件上报"的后台任务模型：
//! `spawn_scan` / `spawn_execute` 返回任务 ID，任务完成后经事件总线广播
//! `TaskStatusChanged` 与 `ScanCompleted` / `ExecutionCompleted` 事件。

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, RwLock};

use aether_core::errors::CapabilityError;
use aether_core::events::{AetherEvent, EventKind};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::bus::EventBus;
use crate::registry::ProviderRegistry;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// 任务类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    Scan,
    Execute,
}

/// 任务信息（对外可见）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInfo {
    pub id: Uuid,
    pub provider_id: String,
    pub kind: TaskKind,
    pub status: TaskStatus,
    pub started_at: DateTime<Utc>,
    pub message: String,
}

/// 内部任务条目
struct TaskEntry {
    info: TaskInfo,
    /// 状态（AtomicU8 映射 TaskStatus）
    status: AtomicU8,
    /// 取消令牌
    cancel: CancellationToken,
}

impl TaskEntry {
    fn set_status(&self, status: TaskStatus) {
        self.status.store(status as u8, Ordering::SeqCst);
    }

    fn current_status(&self) -> TaskStatus {
        match self.status.load(Ordering::SeqCst) {
            0 => TaskStatus::Pending,
            1 => TaskStatus::Running,
            2 => TaskStatus::Completed,
            3 => TaskStatus::Failed,
            _ => TaskStatus::Cancelled,
        }
    }
}

/// 任务调度器
#[derive(Clone)]
pub struct Scheduler {
    registry: Arc<ProviderRegistry>,
    bus: Arc<EventBus>,
    tasks: Arc<RwLock<HashMap<Uuid, Arc<TaskEntry>>>>,
}

impl Scheduler {
    /// 创建调度器
    pub fn new(registry: Arc<ProviderRegistry>, bus: Arc<EventBus>) -> Arc<Self> {
        Arc::new(Self {
            registry,
            bus,
            tasks: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// 后台启动一次 Provider 扫描，立即返回任务 ID
    pub async fn spawn_scan(
        &self,
        provider_id: &str,
        params: Value,
    ) -> Result<Uuid, CapabilityError> {
        let provider = self
            .registry
            .get(provider_id)
            .ok_or_else(|| CapabilityError::NotFound(provider_id.to_string()))?;
        let kind = TaskKind::Scan;
        let task_id = self.insert_task(provider_id, kind);
        let bus = self.bus.clone();
        let pid = provider_id.to_string();

        tokio::spawn(async move {
            let result = provider.scan(params).await;
            finish_task(&bus, &pid, task_id, kind, result);
        });

        Ok(task_id)
    }

    /// 后台启动一次 Provider 动作执行，立即返回任务 ID
    pub async fn spawn_execute(
        &self,
        provider_id: &str,
        action: &str,
        params: Value,
    ) -> Result<Uuid, CapabilityError> {
        let provider = self
            .registry
            .get(provider_id)
            .ok_or_else(|| CapabilityError::NotFound(provider_id.to_string()))?;
        let kind = TaskKind::Execute;
        let task_id = self.insert_task(provider_id, kind);
        let bus = self.bus.clone();
        let pid = provider_id.to_string();
        let action = action.to_string();

        tokio::spawn(async move {
            let result = provider.execute(&action, params).await;
            finish_task(&bus, &pid, task_id, kind, result);
        });

        Ok(task_id)
    }

    /// 登记任务并广播 running 状态，返回 task_id
    fn insert_task(&self, provider_id: &str, kind: TaskKind) -> Uuid {
        let task_id = Uuid::new_v4();
        let entry = Arc::new(TaskEntry {
            info: TaskInfo {
                id: task_id,
                provider_id: provider_id.to_string(),
                kind,
                status: TaskStatus::Running,
                started_at: Utc::now(),
                message: String::new(),
            },
            status: AtomicU8::new(1), // Running
            cancel: CancellationToken::new(),
        });
        self.tasks.write().unwrap().insert(task_id, entry);
        self.emit_status(task_id, provider_id, "running");
        task_id
    }

    fn emit_status(&self, task_id: Uuid, provider_id: &str, status: &str) {
        self.bus.emit(AetherEvent {
            kind: EventKind::StatusChanged,
            source: provider_id.to_string(),
            timestamp_ms: now_ms(),
            payload: serde_json::json!({
                "taskId": task_id,
                "status": status,
            }),
        });
    }

    /// 取消任务
    pub fn cancel(&self, task_id: Uuid) -> Result<(), CapabilityError> {
        let tasks = self.tasks.read().unwrap();
        if let Some(entry) = tasks.get(&task_id) {
            entry.cancel.cancel();
            entry.set_status(TaskStatus::Cancelled);
            Ok(())
        } else {
            Err(CapabilityError::NotFound(format!("task {task_id}")))
        }
    }

    /// 列出所有任务
    pub fn list(&self) -> Vec<TaskInfo> {
        self.tasks
            .read()
            .unwrap()
            .values()
            .map(|e| {
                let mut info = e.info.clone();
                info.status = e.current_status();
                info
            })
            .collect()
    }
}

/// 任务收尾：广播状态/结果事件，并从任务表移除
fn finish_task<R, E>(
    bus: &EventBus,
    provider_id: &str,
    task_id: Uuid,
    kind: TaskKind,
    result: Result<R, E>,
) where
    R: Serialize,
    E: std::fmt::Display,
{
    match result {
        Ok(value) => {
            let event_kind = match kind {
                TaskKind::Scan => EventKind::ScanCompleted,
                TaskKind::Execute => EventKind::ExecutionCompleted,
            };
            bus.emit(AetherEvent {
                kind: event_kind,
                source: provider_id.to_string(),
                timestamp_ms: now_ms(),
                payload: serde_json::to_value(value).unwrap_or_default(),
            });
            bus.emit(AetherEvent {
                kind: EventKind::StatusChanged,
                source: provider_id.to_string(),
                timestamp_ms: now_ms(),
                payload: serde_json::json!({
                    "taskId": task_id,
                    "status": "completed",
                }),
            });
        }
        Err(err) => {
            bus.emit(AetherEvent {
                kind: EventKind::Notification,
                source: provider_id.to_string(),
                timestamp_ms: now_ms(),
                payload: serde_json::json!({
                    "level": "error",
                    "title": "Task failed",
                    "message": err.to_string(),
                }),
            });
        }
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
