//! 领域模型定义：数据实体、值对象、通用数据结构
//!
//! 对齐旧版 C# (AetherOS.Core/Models) 的领域模型，全部为 serde 可序列化结构，
//! 统一 camelCase 命名（枚举值沿用 C# 的 PascalCase 字符串），
//! 作为前后端 IPC 通信的数据契约与旧版 JSON 记录兼容。

pub mod action;
pub mod ai;
pub mod cleaner;
pub mod deep_scan;
pub mod diagnostics;
pub mod hibernate;
pub mod maintenance;
pub mod operation;
pub mod pagefile;
pub mod plugin;
pub mod power;
pub mod quick_scan;
pub mod restore_point;
pub mod settings;
pub mod system_context;
