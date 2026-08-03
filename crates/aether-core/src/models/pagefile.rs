//! 页面文件（虚拟内存）模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 单个页面文件条目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageFileEntry {
    pub path: String,
    pub initial_size_mb: i64,
    pub maximum_size_mb: i64,
    pub current_file_bytes: i64,
    pub exists: bool,
}

/// 页面文件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageFileConfiguration {
    pub is_automatically_managed: bool,
    pub entries: Vec<PageFileEntry>,
    pub total_current_file_bytes: i64,
    pub restart_required: bool,
    pub checked_at: DateTime<Utc>,
}

/// 页面文件变更请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageFileChangeRequest {
    pub path: String,
    pub initial_size_mb: i64,
    pub maximum_size_mb: i64,
}

/// 虚拟内存信息摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMemoryInfo {
    pub is_automatic_management_enabled: bool,
    pub total_page_file_megabytes: i64,
    pub total_physical_memory_megabytes: i64,
    pub summary: String,
}
