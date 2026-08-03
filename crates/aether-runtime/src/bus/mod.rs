//! 事件总线：模块间解耦通信，支持事件广播与订阅
//!
//! 基于 tokio broadcast 实现，Provider/模块通过总线发布事件，
//! 前端可经 Tauri 事件通道订阅转发。

use aether_core::events::{AetherEvent, EventKind};
use tokio::sync::broadcast;

/// 事件订阅通道的接收端
pub type EventReceiver = broadcast::Receiver<AetherEvent>;

/// 事件总线
///
/// 支持多个订阅者；当订阅者消费不及时代数不足时会丢失最旧事件（channel 满自动丢旧）。
#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<AetherEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// 创建事件总线（channel 容量 1024）
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    /// 创建指定容量的事件总线
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self { tx }
    }

    /// 发布事件（广播给所有订阅者）
    pub fn emit(&self, event: AetherEvent) {
        // 忽略无订阅者的错误
        let _ = self.tx.send(event);
    }

    /// 便捷发布：构造事件并广播
    pub fn publish(&self, kind: EventKind, source: &str, payload: serde_json::Value) {
        let event = AetherEvent {
            kind,
            source: source.to_string(),
            timestamp_ms: now_ms(),
            payload,
        };
        self.emit(event);
    }

    /// 订阅事件（返回接收端）
    pub fn subscribe(&self) -> EventReceiver {
        self.tx.subscribe()
    }

    /// 当前订阅者数量
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }
}

/// 获取当前 Unix 时间戳（毫秒）
fn now_ms() -> u64 {
    // Date.now()/SystemTime 在本运行时可用
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
