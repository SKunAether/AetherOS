//! AetherOS 优化能力 Provider
//!
//! 实现电源计划管理、启动项管理、休眠开关、系统服务优化、网络优化、视觉效果调整等业务逻辑。
//! Phase 5 落地：电源计划、休眠、启动项。

pub mod provider;
pub mod startup;

pub use provider::OptimizerProvider;
