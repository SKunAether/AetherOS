//! 应用设置模型（对齐 C# AppSettings）

use serde::{Deserialize, Serialize};

/// 关闭行为
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CloseBehavior {
    /// 最小化到托盘
    MinimizeToTray,
    /// 直接退出
    #[default]
    Exit,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default)]
    pub close_behavior: CloseBehavior,
    /// 开机自启
    #[serde(default)]
    pub run_at_startup: bool,
    /// 每日计划扫描
    #[serde(default)]
    pub enable_scheduled_scan: bool,
    /// 计划扫描时间 "09:00"
    #[serde(default = "default_scan_time")]
    pub scheduled_scan_time: String,
    #[serde(default = "default_true")]
    pub enable_notifications: bool,
    #[serde(default = "default_true")]
    pub check_updates_automatically: bool,
    /// stable / preview
    #[serde(default = "default_channel")]
    pub update_channel: String,
    /// zh-CN / en-US
    #[serde(default = "default_language")]
    pub language: String,
    /// 云端 AI 分析（默认关闭）
    #[serde(default)]
    pub enable_ai_analysis: bool,
    #[serde(default)]
    pub active_ai_provider_id: Option<String>,
}

fn default_scan_time() -> String {
    "09:00".to_string()
}
fn default_true() -> bool {
    true
}
fn default_channel() -> String {
    "stable".to_string()
}
fn default_language() -> String {
    "zh-CN".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            close_behavior: CloseBehavior::Exit,
            run_at_startup: false,
            enable_scheduled_scan: false,
            scheduled_scan_time: default_scan_time(),
            enable_notifications: default_true(),
            check_updates_automatically: default_true(),
            update_channel: default_channel(),
            language: default_language(),
            enable_ai_analysis: false,
            active_ai_provider_id: None,
        }
    }
}
