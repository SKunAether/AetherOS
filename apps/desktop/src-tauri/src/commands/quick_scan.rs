//! 快速扫描命令（Phase 4 实现）

use aether_core::errors::CapabilityError;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// 执行快速扫描（Phase 4 实现）
#[tauri::command]
pub fn get_quick_scan(_state: State<'_, AppState>) -> Result<(), AppError> {
    Err(CapabilityError::NotImplemented("quick_scan.get_quick_scan (Phase 4)".to_string()).into())
}
