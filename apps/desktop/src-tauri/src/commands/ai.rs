//! AI 分析命令：服务商配置 CRUD 与系统分析

use aether_core::models::ai::{AIAnalysisResult, AIProviderConfig};
use serde_json::Value;
use tauri::State;

use crate::ai;
use crate::error::AppError;
use crate::state::AppState;

/// 列出所有 AI 服务商
#[tauri::command]
pub fn get_ai_providers() -> Result<Vec<AIProviderConfig>, AppError> {
    ai::list_providers().map_err(|msg| AppError {
        code: "ai_list_failed".to_string(),
        message: msg,
    })
}

/// 保存 AI 服务商（API Key 加密存储）
#[tauri::command]
pub fn save_ai_provider(provider: AIProviderConfig) -> Result<AIProviderConfig, AppError> {
    ai::save_provider(provider).map_err(|msg| AppError {
        code: "ai_save_failed".to_string(),
        message: msg,
    })
}

/// 删除 AI 服务商
#[tauri::command]
pub fn delete_ai_provider(id: String) -> Result<(), AppError> {
    ai::delete_provider(&id).map_err(|msg| AppError {
        code: "ai_delete_failed".to_string(),
        message: msg,
    })
}

/// 测试 AI 服务商连接
#[tauri::command]
pub fn test_ai_provider(provider: AIProviderConfig) -> Result<bool, AppError> {
    ai::test_provider(&provider).map_err(|msg| AppError {
        code: "ai_test_failed".to_string(),
        message: msg,
    })
}

/// 运行 AI 系统分析（需要已保存的服务商）
#[tauri::command]
pub async fn run_ai_analysis(
    state: State<'_, AppState>,
    provider_id: Option<String>,
) -> Result<AIAnalysisResult, AppError> {
    // 取指定服务商或默认第一个启用的
    let providers = ai::list_providers().map_err(|msg| AppError {
        code: "ai_list_failed".to_string(),
        message: msg,
    })?;
    let provider = provider_id
        .as_deref()
        .and_then(|id| providers.iter().find(|p| p.id == id))
        .or_else(|| providers.iter().find(|p| p.is_enabled))
        .or_else(|| providers.first())
        .ok_or_else(|| AppError {
            code: "ai_no_provider".to_string(),
            message: "请先在设置中配置 AI 服务商".to_string(),
        })?;

    // 采集系统信息构建上下文
    let system_info = state.system.system_info().map_err(AppError::from)?;
    let context = ai::build_system_context(serde_json::to_value(&system_info).unwrap_or_default());

    // 阻塞调用放入 spawn_blocking（网络 I/O）
    let provider = provider.clone();
    let inner = tauri::async_runtime::spawn_blocking(move || ai::run_analysis(&provider, &context))
        .await
        .map_err(|e| AppError {
            code: "ai_runtime_failed".to_string(),
            message: e.to_string(),
        })?;
    inner.map_err(|msg| AppError {
        code: "ai_analysis_failed".to_string(),
        message: msg,
    })
}

/// 供前端查询当前是否已配置 AI
#[tauri::command]
pub fn ai_configured() -> Result<bool, AppError> {
    Ok(!ai::list_providers().unwrap_or_default().is_empty())
}

/// 供前端查询 AI 可用性（无实际调用，保留兼容）
#[tauri::command]
pub fn ai_status() -> Result<Value, AppError> {
    let providers = ai::list_providers().unwrap_or_default();
    Ok(serde_json::json!({
        "configured": !providers.is_empty(),
        "count": providers.len(),
    }))
}
