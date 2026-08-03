//! AetherOS 核心领域层
//!
//! 全项目通用的模型、接口抽象、错误类型与事件定义。
//! 本 crate 不依赖任何平台特性，保证纯粹性与可移植性。

pub mod errors;
pub mod events;
pub mod models;
pub mod traits;
