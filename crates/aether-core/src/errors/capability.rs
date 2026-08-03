//! Capability 层错误类型
//!
//! Provider 与运行时统一使用该错误类型，跨 IPC 边界以 JSON 序列化传递给前端。

use serde::Serialize;
use thiserror::Error;

/// 能力执行错误
#[derive(Debug, Error, Serialize)]
pub enum CapabilityError {
    #[error("capability not found: {0}")]
    NotFound(String),

    #[error("not implemented: {0}")]
    NotImplemented(String),

    #[error("permission denied: {0}")]
    Unauthorized(String),

    #[error("operation cancelled")]
    Cancelled,

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("platform error: {0}")]
    Platform(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for CapabilityError {
    fn from(e: std::io::Error) -> Self {
        CapabilityError::Io(e.to_string())
    }
}
