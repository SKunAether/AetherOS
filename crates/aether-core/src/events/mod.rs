//! 全局事件定义：进度事件、状态变更事件、通知事件
//!
//! 事件通过事件总线（aether-runtime）在模块间解耦通信，并可经由 Tauri
//! 事件通道推送给前端 UI。

use serde::{Deserialize, Serialize};

/// 事件类型标识，前端用于订阅分类
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum EventKind {
    /// 任务进度更新
    Progress,
    /// 任务状态变更
    StatusChanged,
    /// 系统通知
    Notification,
    /// 扫描结果就绪
    ScanCompleted,
    /// 清理/操作执行完成
    ExecutionCompleted,
}

/// 统一事件负载：携带事件类型、来源 Provider、时间与 JSON 数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AetherEvent {
    pub kind: EventKind,
    /// 事件来源（Provider id 或模块名）
    pub source: String,
    /// Unix 时间戳（毫秒）
    pub timestamp_ms: u64,
    /// 事件携带数据（JSON 值）
    pub payload: serde_json::Value,
}
