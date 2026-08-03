//! 应用全局共享状态（通过 tauri::manage 注入，命令用 State 访问）

use std::sync::{Arc, RwLock};

use aether_core::models::settings::AppSettings;
use aether_runtime::RuntimeContext;
use aether_system::SystemEngine;

/// 应用共享状态
pub struct AppState {
    /// 能力运行时（注册中心 + 事件总线 + 调度器）
    pub runtime: Arc<RuntimeContext>,
    /// 系统引擎
    pub system: Arc<SystemEngine>,
    /// 应用设置（可写，读多写少）
    pub settings: Arc<RwLock<AppSettings>>,
}
