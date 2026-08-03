//! 深度扫描结果模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 文件类型统计（按扩展名聚合）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTypeStat {
    pub extension: String,
    pub total_bytes: i64,
    pub file_count: i64,
    /// 占比（服务端计算，0-100）
    pub percentage: f64,
}

/// 大文件信息（Top 100）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeFileInfo {
    pub full_path: String,
    pub size_bytes: i64,
    pub extension: String,
    pub last_modified: DateTime<Utc>,
}

/// 深度扫描结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepScanResult {
    pub root_path: String,
    pub total_bytes: i64,
    pub total_file_count: i64,
    pub total_directory_count: i64,
    pub skipped_directory_count: i64,
    pub file_type_stats: Vec<FileTypeStat>,
    pub largest_files: Vec<LargeFileInfo>,
    #[serde(default)]
    pub is_cancelled: bool,
    pub completed_at: DateTime<Utc>,
    /// 耗时毫秒
    pub elapsed_ms: i64,
}
