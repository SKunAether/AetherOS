//! 清理 Provider：扫描与执行系统清理
//!
//! 规则表对齐旧版 C# `WindowsCleanerScanService` / `WindowsCleanerExecutionService`：
//! - cleaner.user-temp: %TEMP%（低风险，默认选中）
//! - cleaner.windows-temp: %WINDIR%\Temp（低风险，需管理员，默认选中）
//! - cleaner.update-download: %WINDIR%\SoftwareDistribution\Download（中风险，需管理员）
//! - cleaner.thumbnail-cache: %LOCALAPPDATA%\Microsoft\Windows\Explorer（低风险，thumbcache_* 过滤）

use std::path::PathBuf;
use std::time::Duration;

use aether_core::errors::CapabilityError;
use aether_core::models::cleaner::{
    CleanerExecutionItemResult, CleanerExecutionResult, CleanerItem, CleanerRiskLevel,
    CleanerScanResult,
};
use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType, ScanResult};
use aether_system::filesystem;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;

/// 清理规则定义
#[derive(Debug, Clone)]
pub struct CleanerRule {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub path: PathBuf,
    pub risk: CleanerRiskLevel,
    pub requires_admin: bool,
    pub default_selected: bool,
    /// 文件过滤器（可选）
    pub file_filter: Option<fn(&std::path::Path) -> bool>,
}

fn is_thumbcache(p: &std::path::Path) -> bool {
    p.file_name()
        .map(|n| n.to_string_lossy().starts_with("thumbcache_"))
        .unwrap_or(false)
}

/// 构建规则表（对齐 C# CreateAllowedTargets）
fn rules() -> Vec<CleanerRule> {
    let temp = std::env::var("TEMP")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
    let localappdata = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| temp.clone());

    vec![
        CleanerRule {
            id: "cleaner.user-temp",
            name: "用户临时文件",
            description: "当前用户临时目录中的临时文件",
            path: PathBuf::from(&temp),
            risk: CleanerRiskLevel::Low,
            requires_admin: false,
            default_selected: true,
            file_filter: None,
        },
        CleanerRule {
            id: "cleaner.windows-temp",
            name: "Windows 临时文件",
            description: "系统临时目录中的临时文件（需管理员）",
            path: PathBuf::from(&windir).join("Temp"),
            risk: CleanerRiskLevel::Low,
            requires_admin: true,
            default_selected: true,
            file_filter: None,
        },
        CleanerRule {
            id: "cleaner.update-download",
            name: "Windows 更新缓存",
            description: "Windows 更新下载缓存（需管理员）",
            path: PathBuf::from(&windir)
                .join("SoftwareDistribution")
                .join("Download"),
            risk: CleanerRiskLevel::Medium,
            requires_admin: true,
            default_selected: false,
            file_filter: None,
        },
        CleanerRule {
            id: "cleaner.thumbnail-cache",
            name: "缩略图缓存",
            description: "资源管理器缩略图缓存",
            path: PathBuf::from(&localappdata)
                .join("Microsoft")
                .join("Windows")
                .join("Explorer"),
            risk: CleanerRiskLevel::Low,
            requires_admin: false,
            default_selected: false,
            file_filter: Some(is_thumbcache),
        },
    ]
}

/// 清理 Provider
pub struct CleanerProvider;

impl Default for CleanerProvider {
    fn default() -> Self {
        Self
    }
}

impl CleanerProvider {
    /// 创建清理 Provider
    pub fn new() -> Self {
        Self
    }

    /// 扫描所有规则项，估算可释放空间
    pub fn scan_items(&self) -> CleanerScanResult {
        let mut items = Vec::new();
        for rule in rules() {
            let estimated = if rule.path.exists() {
                filesystem::dir_size_bytes(&rule.path, Duration::from_secs(4))
            } else {
                0
            };
            items.push(CleanerItem {
                id: rule.id.to_string(),
                name: rule.name.to_string(),
                description: rule.description.to_string(),
                path: rule.path.to_string_lossy().to_string(),
                estimated_bytes: estimated,
                risk_level: rule.risk,
                requires_administrator: rule.requires_admin,
                is_selected_by_default: rule.default_selected,
            });
        }
        let total = items.iter().map(|i| i.estimated_bytes).sum();
        CleanerScanResult {
            items,
            total_estimated_bytes: total,
            completed_at: Utc::now(),
        }
    }

    /// 执行清理：仅处理白名单内且存在的规则；测 Before → 删除 → 测 After
    pub fn execute(&self, selected_ids: &[String]) -> CleanerExecutionResult {
        let mut results = Vec::new();
        for rule in rules() {
            if !selected_ids.iter().any(|id| id == rule.id) {
                continue;
            }
            if !rule.path.exists() {
                results.push(CleanerExecutionItemResult {
                    item_id: rule.id.to_string(),
                    name: rule.name.to_string(),
                    path: rule.path.to_string_lossy().to_string(),
                    before_bytes: 0,
                    after_bytes: 0,
                    released_bytes: 0,
                    deleted_file_count: 0,
                    skipped_file_count: 0,
                    succeeded: true,
                    requires_administrator: rule.requires_admin,
                    error_message: None,
                });
                continue;
            }

            let before = filesystem::dir_size_bytes(&rule.path, Duration::from_secs(4));
            let (deleted, skipped) = delete_under(&rule.path, rule.file_filter);
            let after = filesystem::dir_size_bytes(&rule.path, Duration::from_secs(2));

            results.push(CleanerExecutionItemResult {
                item_id: rule.id.to_string(),
                name: rule.name.to_string(),
                path: rule.path.to_string_lossy().to_string(),
                before_bytes: before,
                after_bytes: after,
                released_bytes: before - after,
                deleted_file_count: deleted,
                skipped_file_count: skipped,
                succeeded: true,
                requires_administrator: rule.requires_admin,
                error_message: None,
            });
        }

        let total_released = results.iter().map(|r| r.released_bytes).sum();
        let total_deleted = results.iter().map(|r| r.deleted_file_count).sum();
        let total_skipped = results.iter().map(|r| r.skipped_file_count).sum();
        CleanerExecutionResult {
            items: results,
            total_released_bytes: total_released,
            total_deleted_file_count: total_deleted,
            total_skipped_file_count: total_skipped,
            succeeded: true,
            completed_at: Utc::now(),
        }
    }
}

/// 递归删除目录下符合过滤器的文件（跳过重解析点与目录本身），返回 (删除数, 跳过数)
fn delete_under(
    root: &std::path::Path,
    filter: Option<fn(&std::path::Path) -> bool>,
) -> (i64, i64) {
    let mut deleted = 0i64;
    let mut skipped = 0i64;
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if filesystem::is_reparse_point(&dir) {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                if is_dir {
                    if !filesystem::is_reparse_point(&path) {
                        stack.push(path);
                    } else {
                        skipped += 1;
                    }
                    continue;
                }
                let matches = filter.map(|f| f(&path)).unwrap_or(true);
                if matches {
                    if filesystem::delete_file_force(&path).is_ok() {
                        deleted += 1;
                    } else {
                        skipped += 1;
                    }
                } else {
                    skipped += 1;
                }
            }
        }
    }
    (deleted, skipped)
}

#[async_trait]
impl CapabilityProvider for CleanerProvider {
    fn id(&self) -> &str {
        "aether.cleaner"
    }

    fn name(&self) -> &str {
        "系统清理"
    }

    fn description(&self) -> &str {
        "临时文件、Windows 更新缓存、缩略图缓存清理"
    }

    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Cleaner
    }

    fn actions(&self) -> Vec<ActionDef> {
        vec![
            ActionDef {
                id: "scan".to_string(),
                name: "扫描".to_string(),
                description: "扫描可清理项".to_string(),
                risk_level: CleanerRiskLevel::Low,
                requires_administrator: false,
                is_reversible: false,
            },
            ActionDef {
                id: "execute".to_string(),
                name: "执行清理".to_string(),
                description: "删除已勾选的清理项".to_string(),
                risk_level: CleanerRiskLevel::Medium,
                requires_administrator: false,
                is_reversible: false,
            },
        ]
    }

    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError> {
        match action {
            "scan" => {
                let result = self.scan_items();
                serde_json::to_value(result).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            "execute" => {
                let selected_ids: Vec<String> = params
                    .get("selectedIds")
                    .and_then(Value::as_array)
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let result = self.execute(&selected_ids);
                serde_json::to_value(result).map_err(|e| CapabilityError::Internal(e.to_string()))
            }
            other => Err(CapabilityError::NotImplemented(format!(
                "cleaner action '{other}'"
            ))),
        }
    }

    async fn scan(&self, _params: Value) -> Result<ScanResult, CapabilityError> {
        let result = self.scan_items();
        Ok(ScanResult {
            provider_id: self.id().to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            data: serde_json::to_value(result)
                .map_err(|e| CapabilityError::Internal(e.to_string()))?,
            error: None,
            cancelled: false,
        })
    }
}
