//! AetherOS 插件开发 SDK
//!
//! 封装插件开发所需的所有接口、类型与工具（含 PluginManifest 等），
//! 第三方插件通过实现 CapabilityProvider 接口即可无缝接入系统。
//!
//! 使用：`use aether_plugin_sdk::prelude::*;`

pub mod prelude {
    //! 插件开发的统一入口：re-export 核心 trait 与类型
    pub use aether_core::errors::CapabilityError;
    pub use aether_core::events::{AetherEvent, EventKind};
    pub use aether_core::models::cleaner::CleanerRiskLevel;
    pub use aether_core::models::plugin::PluginManifest;
    pub use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
    pub use async_trait::async_trait;
    pub use serde_json::{json, Value};
}

/// SDK 版本
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
