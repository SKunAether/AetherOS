//! IPC 命令定义：暴露给前端调用的所有后端接口入口
//!
//! 按功能域拆分子模块，在 main.rs 统一注册到 invoke_handler。

pub mod ai;
pub mod cleaner;
pub mod hibernate;
pub mod power;
pub mod quick_scan;
pub mod runtime;
pub mod settings;
pub mod system;
