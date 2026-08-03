//! 插件清单模型（对齐 C# PluginManifest）
//!
//! 声明式插件：插件 = 含 plugin.json 的 ZIP 包，不执行任意代码。

use serde::{Deserialize, Serialize};

/// 插件清单
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    /// Rule Pack 等
    pub category: String,
    pub minimum_aether_os_version: String,
    /// 能力声明，如 ["scan.temp", "scan.thumbnail"]
    pub capabilities: Vec<String>,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
    #[serde(default)]
    pub is_built_in: bool,
    /// 运行时填充字段
    pub manifest_path: String,
    pub install_directory: String,
    pub is_compatible: bool,
    pub compatibility_message: String,
    /// 计算状态文本（服务端计算）
    pub status_text: String,
}

fn default_true() -> bool {
    true
}
