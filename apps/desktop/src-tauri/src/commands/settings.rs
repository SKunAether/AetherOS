//! 设置命令：读写 %LOCALAPPDATA%\AetherOS\Settings\appsettings.json

use aether_core::models::settings::AppSettings;
use tauri::State;

use crate::error::AppError;
use crate::settings;
use crate::state::AppState;

/// 获取当前设置
#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    let guard = state.settings.read().unwrap();
    Ok(guard.clone())
}

/// 保存设置（写盘并广播 SettingsChanged 事件）
#[tauri::command]
pub fn save_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<AppSettings, AppError> {
    // 同步开机自启注册
    if let Ok(exe) = std::env::current_exe() {
        crate::startup::apply(&exe.to_string_lossy(), settings.run_at_startup).map_err(|msg| {
            AppError {
                code: "startup_apply_failed".to_string(),
                message: msg,
            }
        })?;
    }
    {
        let mut guard = state.settings.write().unwrap();
        *guard = settings.clone();
    }
    settings::save(&settings, &state.runtime).map_err(|msg| AppError {
        code: "settings_save_failed".to_string(),
        message: msg,
    })?;
    Ok(settings)
}
