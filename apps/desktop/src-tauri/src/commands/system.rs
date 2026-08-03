//! 系统信息命令：真实实现（经 aether-system 采集）

use aether_system::SystemInfo;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// 获取系统信息（CPU/内存/磁盘/OS/管理员状态）
#[tauri::command]
pub fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, AppError> {
    state.system.system_info().map_err(AppError::from)
}

/// 用系统默认浏览器打开外部链接
#[tauri::command]
pub fn open_external(app: tauri::AppHandle, url: String) -> Result<(), AppError> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| AppError {
            code: "open_external_failed".to_string(),
            message: e.to_string(),
        })
}
