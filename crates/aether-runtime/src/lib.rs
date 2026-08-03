//! AetherOS 能力运行时
//!
//! 系统调度中枢：Provider 注册发现、异步任务调度、插件生命周期管理、事件广播。
//! `RuntimeContext` 是顶层门面，组装注册中心、事件总线与调度器，
//! 由桌面壳层（src-tauri）创建并通过 `tauri::manage` 注入。

pub mod bus;
pub mod plugin;
pub mod registry;
pub mod scheduler;

use std::sync::Arc;

use bus::EventBus;
use registry::ProviderRegistry;
use scheduler::Scheduler;

/// 运行时上下文：注册中心 + 事件总线 + 调度器
#[derive(Clone)]
pub struct RuntimeContext {
    pub registry: Arc<ProviderRegistry>,
    pub bus: Arc<EventBus>,
    pub scheduler: Arc<Scheduler>,
}

impl RuntimeContext {
    /// 创建运行时上下文（三者互相引用，统一在此组装）
    pub fn bootstrap() -> Arc<Self> {
        let registry = Arc::new(ProviderRegistry::new());
        let bus = Arc::new(EventBus::new());
        let scheduler = Scheduler::new(registry.clone(), bus.clone());
        Arc::new(Self {
            registry,
            bus,
            scheduler,
        })
    }
}
