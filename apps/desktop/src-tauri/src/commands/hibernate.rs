//! 休眠命令
//!
//! get_hibernate_state / set_hibernate_enabled 将在 Phase 5（优化模块）实现真实逻辑，
//! 当前返回 NotImplemented 占位。

use aether_core::errors::CapabilityError;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// 获取休眠状态（Phase 5 实现）
#[tauri::command]
pub fn get_hibernate_state(_state: State<'_, AppState>) -> Result<(), AppError> {
    Err(
        CapabilityError::NotImplemented("hibernate.get_hibernate_state (Phase 5)".to_string())
            .into(),
    )
}

/// 开启/关闭休眠（Phase 5 实现，需提权）
#[tauri::command]
pub fn set_hibernate_enabled(_state: State<'_, AppState>, _enabled: bool) -> Result<(), AppError> {
    Err(
        CapabilityError::NotImplemented("hibernate.set_hibernate_enabled (Phase 5)".to_string())
            .into(),
    )
}
