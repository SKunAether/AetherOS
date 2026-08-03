//! 核心接口抽象：CapabilityProvider 通用接口、服务基础接口
//!
//! 所有系统能力均以标准化 Provider 形式提供，是 AetherOS 架构的核心抽象。
//! 注册中心以 `Arc<dyn CapabilityProvider>` 存储，因此 trait 必须 object-safe：
//! 返回类型统一使用 serde_json::Value，参数与结果跨 IPC 以 JSON 传递。

pub mod capability_provider;

pub use capability_provider::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
