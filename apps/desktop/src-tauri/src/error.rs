//! 命令层统一错误类型：跨 IPC 以 JSON 序列化返回给前端

use serde::Serialize;
use thiserror::Error;

/// 命令层错误（code + message，前端按 code 分支处理）
#[derive(Debug, Serialize, Error)]
#[error("{message}")]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
        }
    }
}

impl From<aether_core::errors::CapabilityError> for AppError {
    fn from(e: aether_core::errors::CapabilityError) -> Self {
        let code = match &e {
            aether_core::errors::CapabilityError::NotFound(_) => "not_found",
            aether_core::errors::CapabilityError::NotImplemented(_) => "not_implemented",
            aether_core::errors::CapabilityError::Unauthorized(_) => "unauthorized",
            aether_core::errors::CapabilityError::Cancelled => "cancelled",
            aether_core::errors::CapabilityError::InvalidArgument(_) => "invalid_argument",
            aether_core::errors::CapabilityError::Io(_) => "io_error",
            aether_core::errors::CapabilityError::Platform(_) => "platform_error",
            aether_core::errors::CapabilityError::Internal(_) => "internal_error",
        };
        Self::new(code, e.to_string())
    }
}
