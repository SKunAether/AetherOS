//! 应用设置持久化：读写 %LOCALAPPDATA%\AetherOS\Settings\appsettings.json

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use aether_core::models::settings::AppSettings;
use aether_runtime::RuntimeContext;

/// 设置存储路径（对齐旧版 C# 布局）
pub fn settings_file() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(base)
        .join("AetherOS")
        .join("Settings")
        .join("appsettings.json")
}

/// 加载设置（文件不存在返回默认值）
pub fn load() -> AppSettings {
    let path = settings_file();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// 保存设置（自动创建目录），并广播 SettingsChanged 事件
pub fn save(settings: &AppSettings, runtime: &Arc<RuntimeContext>) -> Result<(), String> {
    let path = settings_file();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    runtime.bus.publish(
        aether_core::events::EventKind::StatusChanged,
        "settings",
        serde_json::json!({
            "event": "settingsChanged",
            "language": settings.language,
            "updateChannel": settings.update_channel,
        }),
    );
    Ok(())
}

/// 创建一个带读写锁的设置句柄
pub fn handle(settings: AppSettings) -> Arc<RwLock<AppSettings>> {
    Arc::new(RwLock::new(settings))
}
