//! 清理命令（Phase 4 实现）

use aether_core::errors::CapabilityError;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// 执行清理扫描（Phase 4 实现）
#[tauri::command]
pub fn cleaner_scan(_state: State<'_, AppState>) -> Result<(), AppError> {
    Err(CapabilityError::NotImplemented("cleaner.cleaner_scan (Phase 4)".to_string()).into())
}

/// 执行清理（Phase 4 实现）
#[tauri::command]
pub fn cleaner_execute(
    _state: State<'_, AppState>,
    _selected_ids: Vec<String>,
) -> Result<(), AppError> {
    Err(CapabilityError::NotImplemented("cleaner.cleaner_execute (Phase 4)".to_string()).into())
}
