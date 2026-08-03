//! AetherOS 清理能力 Provider
//!
//! 实现临时文件清理、浏览器缓存清理、系统日志清理、回收站清理、注册表垃圾清理等业务逻辑。
//! Phase 4 落地：临时文件（user-temp/windows-temp）、更新缓存（update-download）、缩略图缓存（thumbnail-cache）。

pub mod provider;

pub use provider::CleanerProvider;
