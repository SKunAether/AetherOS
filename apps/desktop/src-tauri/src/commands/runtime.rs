//! 运行时命令：Provider 列表、调度执行、事件测试
//!
//! 这些命令证明 CapabilityProvider 全链路：前端调用 → IPC → 运行时调度 → Provider 执行。

use aether_core::traits::ScanResult;
use aether_runtime::registry::ProviderDescriptor;
use serde_json::Value;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// 列出所有已注册 Provider
#[tauri::command]
pub fn get_providers(state: State<'_, AppState>) -> Result<Vec<ProviderDescriptor>, AppError> {
    Ok(state.runtime.registry.descriptors())
}

/// 同步执行一次 Provider 动作（走运行时调度）
#[tauri::command]
pub async fn provider_execute(
    state: State<'_, AppState>,
    provider_id: String,
    action: String,
    params: Value,
) -> Result<Value, AppError> {
    let provider = state.runtime.registry.get(&provider_id).ok_or_else(|| {
        AppError::from(aether_core::errors::CapabilityError::NotFound(
            provider_id.clone(),
        ))
    })?;
    provider
        .execute(&action, params)
        .await
        .map_err(AppError::from)
}

/// 执行一次 Provider 扫描（走运行时调度）
#[tauri::command]
pub async fn provider_scan(
    state: State<'_, AppState>,
    provider_id: String,
    params: Value,
) -> Result<ScanResult, AppError> {
    let provider = state.runtime.registry.get(&provider_id).ok_or_else(|| {
        AppError::from(aether_core::errors::CapabilityError::NotFound(
            provider_id.clone(),
        ))
    })?;
    provider.scan(params).await.map_err(AppError::from)
}

/// 链路探测：返回 "pong"
#[tauri::command]
pub fn ping() -> Result<String, AppError> {
    Ok("pong".to_string())
}

/// 事件广播测试：向前端推送一条通知事件
#[tauri::command]
pub fn emit_test(state: State<'_, AppState>, payload: Value) -> Result<(), AppError> {
    state.runtime.bus.publish(
        aether_core::events::EventKind::Notification,
        "test",
        payload,
    );
    Ok(())
}
