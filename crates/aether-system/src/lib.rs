//! AetherOS 系统引擎层
//!
//! 封装所有 Windows 底层 API（注册表、进程、服务、文件系统、硬件信息、电源），
//! 向上层 Provider 提供安全统一的系统操作接口，便于后续跨平台适配。
//! 设计原则：所有底层操作统一收敛到该层，上层 Provider 不直接调用 Windows API。

pub mod filesystem;
pub mod hardware;
pub mod power;
pub mod process;
pub mod registry;
pub mod service;

use std::sync::Arc;

pub use hardware::SystemInfo;

/// 系统引擎门面：无状态句柄，方法内部调用各子模块
#[derive(Clone, Default)]
pub struct SystemEngine {}

impl SystemEngine {
    /// 创建系统引擎
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    /// 采集系统信息（CPU/内存/磁盘/OS/管理员状态）
    pub fn system_info(&self) -> Result<SystemInfo, aether_core::errors::CapabilityError> {
        hardware::collect()
    }

    /// 电源计划列表
    pub fn power_plans(
        &self,
    ) -> Result<Vec<power::PowerPlan>, aether_core::errors::CapabilityError> {
        power::list_plans()
    }

    /// 激活电源计划（需管理员）
    pub fn activate_power_plan(
        &self,
        id: uuid::Uuid,
    ) -> Result<i32, aether_core::errors::CapabilityError> {
        power::activate_plan(id)
    }

    /// 开关休眠（需管理员）
    pub fn set_hibernate(
        &self,
        enabled: bool,
    ) -> Result<i32, aether_core::errors::CapabilityError> {
        power::set_hibernate(enabled)
    }

    /// 当前进程是否管理员
    pub fn is_elevated(&self) -> Result<bool, aether_core::errors::CapabilityError> {
        hardware::collect().map(|info| info.is_administrator)
    }
}
