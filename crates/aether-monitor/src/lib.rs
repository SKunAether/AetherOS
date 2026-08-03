//! AetherOS 监控能力 Provider
//!
//! 实现 CPU / 内存 / 磁盘 / 系统实时监控采样，健康状态评分与数据采集。

pub mod provider;

pub use provider::MonitorProvider;
