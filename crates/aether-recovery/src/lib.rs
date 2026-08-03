//! AetherOS 恢复能力 Provider
//!
//! 实现系统还原点管理、配置备份与恢复、操作回滚等业务逻辑。
//! Phase 5 落地：操作历史（JSON 持久化）与系统还原点。

pub mod history;
pub mod provider;

pub use provider::RecoveryProvider;
