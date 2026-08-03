//! 内置 Provider 组装：注册随主程序发布的官方能力 Provider

pub mod demo;

use std::sync::Arc;

use aether_cleaner::CleanerProvider;
use aether_monitor::MonitorProvider;
use aether_optimizer::OptimizerProvider;
use aether_recovery::RecoveryProvider;
use aether_runtime::registry::ProviderRegistry;
use aether_system::SystemEngine;

/// 注册所有内置 Provider（演示 + 清理 + 监控 + 优化 + 恢复）
pub fn register_builtin(registry: &Arc<ProviderRegistry>, system: &Arc<SystemEngine>) {
    registry.register(demo::DemoProvider::new());
    registry.register(CleanerProvider::new());
    registry.register(MonitorProvider::new(system.clone()));
    registry.register(OptimizerProvider::new(system.clone()));
    registry.register(RecoveryProvider::new());
    tracing::info!("built-in providers registered: {:?}", registry.ids());
}
