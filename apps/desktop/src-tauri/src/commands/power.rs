//! 电源计划命令
//!
//! get_power_plans / activate_power_plan 将在 Phase 5（优化模块）接入 aether-system 的
//! powercfg 封装后实现真实逻辑，当前返回 NotImplemented 占位。

use aether_core::errors::CapabilityError;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// 获取电源计划列表（Phase 5 实现）
#[tauri::command]
pub fn get_power_plans(_state: State<'_, AppState>) -> Result<(), AppError> {
    Err(CapabilityError::NotImplemented("power.get_power_plans (Phase 5)".to_string()).into())
}

/// 激活指定电源计划（Phase 5 实现，需提权）
#[tauri::command]
pub fn activate_power_plan(_state: State<'_, AppState>, _plan_id: String) -> Result<(), AppError> {
    Err(CapabilityError::NotImplemented("power.activate_power_plan (Phase 5)".to_string()).into())
}
