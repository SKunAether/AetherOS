//! 文件系统操作：文件扫描、批量删除、权限管理
//!
//! 对齐旧版 C# WindowsCleanerScanService / WindowsCleanerExecutionService 的算法：
//! 时间预算扫描、跳过重解析点、吞 IO/Unauthorized 异常、清只读位后删除。

use std::path::Path;
use std::time::{Duration, Instant};

use aether_core::errors::CapabilityError;

/// 文件过滤器类型
pub type FileFilter = Box<dyn Fn(&Path) -> bool + Send + Sync>;

/// 目录扫描选项
pub struct DirScanOptions {
    /// 单目录时间预算
    pub budget: Duration,
    /// 文件过滤器（如 thumbcache_* 前缀）
    pub file_filter: Option<FileFilter>,
}

impl Default for DirScanOptions {
    fn default() -> Self {
        Self {
            budget: Duration::from_secs(4),
            file_filter: None,
        }
    }
}

/// 目录扫描结果
#[derive(Debug, Clone)]
pub struct DirScanResult {
    pub total_bytes: i64,
    pub file_count: i64,
    pub timed_out: bool,
}

/// 栈式迭代 + 时间预算 + 跳过重解析点 + 吞 IO/Unauthorized 异常。
/// 算法对齐 C# `WindowsCleanerScanService`（`ScanDirectorySize`）。
pub fn measure_dir_size(root: &Path, opts: &DirScanOptions) -> DirScanResult {
    let started = Instant::now();
    let mut total_bytes = 0i64;
    let mut file_count = 0i64;
    let mut timed_out = false;

    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if started.elapsed() > opts.budget {
            timed_out = true;
            break;
        }

        // 跳过重解析点（junction/symlink），防止循环递归
        if is_reparse_point(&dir) {
            continue;
        }

        // 枚举文件
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if started.elapsed() > opts.budget {
                    timed_out = true;
                    break;
                }
                let path = entry.path();
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

                if is_dir {
                    if !is_reparse_point(&path) {
                        stack.push(path);
                    }
                } else if let Ok(meta) = entry.metadata() {
                    let matches = opts.file_filter.as_ref().map(|f| f(&path)).unwrap_or(true);
                    if matches {
                        total_bytes += meta.len() as i64;
                        file_count += 1;
                    }
                }
            }
        }
        // 吞掉所有 IO/权限错误，与 C# 行为一致
    }

    DirScanResult {
        total_bytes,
        file_count,
        timed_out,
    }
}

/// 判断路径是否为重解析点（junction/symlink/云占位文件）。
/// 用 GetFileAttributesW 判定 FILE_ATTRIBUTE_REPARSE_POINT，覆盖 std 无法识别的场景。
pub fn is_reparse_point(path: &Path) -> bool {
    use windows::core::HSTRING;
    use windows::Win32::Storage::FileSystem::{
        GetFileAttributesW, FILE_ATTRIBUTE_REPARSE_POINT, INVALID_FILE_ATTRIBUTES,
    };

    let path_str = path.to_string_lossy();
    let attrs = unsafe {
        GetFileAttributesW(windows::core::PCWSTR(
            HSTRING::from(path_str.as_ref()).as_ptr(),
        ))
    };
    if attrs == INVALID_FILE_ATTRIBUTES {
        return false;
    }
    attrs & FILE_ATTRIBUTE_REPARSE_POINT.0 != 0
}

/// 校验候选路径是否在根目录内（大小写不敏感前缀 + 边界检查，防路径越界）。
/// 对齐 C# `IsInsideRoot`。
pub fn is_inside_root(candidate: &Path, root: &Path) -> bool {
    let cand = normalize_for_compare(candidate);
    let root_norm = normalize_for_compare(root);
    // 前缀匹配 + 边界检查：根之后必须是分隔符或结尾，避免 "Temp2" 误判为 "Temp" 的子路径
    cand == root_norm
        || (cand.starts_with(&root_norm)
            && cand
                .as_bytes()
                .get(root_norm.len())
                .map(|&b| b == b'\\')
                .unwrap_or(false))
}

fn normalize_for_compare(p: &Path) -> String {
    let s = p.to_string_lossy().replace('/', "\\");
    let s = s.trim_end_matches('\\').to_string();
    s.to_lowercase()
}

/// 清除只读位（对齐 C# `File.SetAttributes(attrs & ~ReadOnly)`）
pub fn clear_readonly(p: &Path) -> std::io::Result<()> {
    use windows::core::HSTRING;
    use windows::Win32::Storage::FileSystem::{
        GetFileAttributesW, SetFileAttributesW, FILE_ATTRIBUTE_READONLY, FILE_FLAGS_AND_ATTRIBUTES,
        INVALID_FILE_ATTRIBUTES,
    };

    let path_str = p.to_string_lossy();
    let ptr = windows::core::PCWSTR(HSTRING::from(path_str.as_ref()).as_ptr());
    let attrs = unsafe { GetFileAttributesW(ptr) };
    if attrs == INVALID_FILE_ATTRIBUTES {
        return Err(std::io::Error::last_os_error());
    }
    let cleared = attrs & !FILE_ATTRIBUTE_READONLY.0;
    if cleared != attrs {
        unsafe {
            SetFileAttributesW(ptr, FILE_FLAGS_AND_ATTRIBUTES(cleared))
                .map_err(|_| std::io::Error::last_os_error())?;
        }
    }
    Ok(())
}

/// 强制删除单个文件（清只读位后删除）
pub fn delete_file_force(p: &Path) -> std::io::Result<()> {
    let _ = clear_readonly(p);
    std::fs::remove_file(p)
}

/// 递归删除目录树（清只读位，吞权限错误，最后删空目录）
pub fn delete_tree_force(p: &Path) -> std::io::Result<()> {
    if !p.exists() {
        return Ok(());
    }
    // 先删文件
    if let Ok(entries) = std::fs::read_dir(p) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    if !is_reparse_point(&path) {
                        let _ = delete_tree_force(&path);
                    }
                } else {
                    let _ = delete_file_force(&path);
                }
            }
        }
    }
    // 再删目录本身
    let _ = clear_readonly(p);
    let _ = std::fs::remove_dir(p);
    Ok(())
}

/// 测量目录总大小（字节），带时间预算
pub fn dir_size_bytes(root: &Path, budget: Duration) -> i64 {
    measure_dir_size(
        root,
        &DirScanOptions {
            budget,
            file_filter: None,
        },
    )
    .total_bytes
}

/// 计算路径在根下（含空目录清理场景），对齐 C# `IsInsideRoot`
pub fn guard_inside_root(candidate: &Path, root: &Path) -> Result<(), CapabilityError> {
    if is_inside_root(candidate, root) {
        Ok(())
    } else {
        Err(CapabilityError::InvalidArgument(format!(
            "path '{}' is outside root '{}'",
            candidate.display(),
            root.display()
        )))
    }
}
