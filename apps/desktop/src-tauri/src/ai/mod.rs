//! AI 分析服务：多厂商配置（DPAPI 加密 API Key）与系统上下文分析
//!
//! 对齐旧版 C# `AIProviderService` / `AIAnalysisService`。
//! 支持 OpenAI 兼容接口（DeepSeek/Qwen/Groq/Gemini 等）与 Anthropic Claude。

use std::path::PathBuf;

use aether_core::models::ai::{AIAnalysisResult, AIProviderConfig, AIProviderType};
use base64::Engine;
use serde_json::{json, Value};

/// 配置文件路径：%LOCALAPPDATA%\AetherOS\Settings\providers.json
pub fn providers_file() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(base)
        .join("AetherOS")
        .join("Settings")
        .join("providers.json")
}

// ---- DPAPI 加密（Windows 数据保护 API）----

/// 使用 DPAPI 加密字符串（当前用户作用域）
pub fn dpapi_encrypt(plain: &str) -> Result<String, String> {
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Security::Cryptography::{
        CryptProtectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
    };
    let bytes = plain.as_bytes();
    let blob = CRYPT_INTEGER_BLOB {
        pbData: bytes.as_ptr() as *mut u8,
        cbData: bytes.len() as u32,
    };
    let mut out = CRYPT_INTEGER_BLOB::default();
    let ok = unsafe {
        CryptProtectData(
            &blob,
            windows::core::w!("AetherOS"),
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut out,
        )
    };
    if ok.is_err() {
        return Err("DPAPI 加密失败".to_string());
    }
    let encrypted = unsafe { std::slice::from_raw_parts(out.pbData, out.cbData as usize) };
    let b64 = base64::engine::general_purpose::STANDARD.encode(encrypted);
    unsafe {
        LocalFree(Some(windows::Win32::Foundation::HLOCAL(out.pbData as _)));
    }
    Ok(b64)
}

/// 使用 DPAPI 解密
pub fn dpapi_decrypt(b64: &str) -> Result<String, String> {
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};
    let encrypted = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| format!("base64 解码失败: {e}"))?;
    let blob = CRYPT_INTEGER_BLOB {
        pbData: encrypted.as_ptr() as *mut u8,
        cbData: encrypted.len() as u32,
    };
    let mut out = CRYPT_INTEGER_BLOB::default();
    let ok = unsafe {
        CryptUnprotectData(
            &blob,
            None,
            None,
            None,
            None,
            windows::Win32::Security::Cryptography::CRYPTPROTECT_UI_FORBIDDEN,
            &mut out,
        )
    };
    if ok.is_err() {
        return Err("DPAPI 解密失败".to_string());
    }
    let plain = unsafe { std::slice::from_raw_parts(out.pbData, out.cbData as usize) };
    let text = String::from_utf8_lossy(plain).to_string();
    unsafe {
        LocalFree(Some(windows::Win32::Foundation::HLOCAL(out.pbData as _)));
    }
    Ok(text)
}

// ---- Provider 配置 CRUD ----

/// 读取所有 AI 服务商配置（脱敏：不解密 Key 给前端）
pub fn list_providers() -> Result<Vec<AIProviderConfig>, String> {
    let path = providers_file();
    let providers = std::fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Vec<AIProviderConfig>>(&t).ok())
        .unwrap_or_default();
    Ok(providers)
}

/// 保存服务商配置（API Key 用 DPAPI 加密存储）
pub fn save_provider(mut provider: AIProviderConfig) -> Result<AIProviderConfig, String> {
    // 仅加密非空且未加密的 Key
    if !provider.encrypted_api_key.is_empty() && !provider.encrypted_api_key.starts_with("dpapi:") {
        let encrypted = dpapi_encrypt(&provider.encrypted_api_key)?;
        provider.encrypted_api_key = format!("dpapi:{encrypted}");
    }
    let mut providers = list_providers()?;
    if let Some(existing) = providers.iter_mut().find(|p| p.id == provider.id) {
        *existing = provider.clone();
    } else {
        providers.push(provider.clone());
    }
    persist(&providers)?;
    Ok(provider)
}

/// 删除服务商配置
pub fn delete_provider(id: &str) -> Result<(), String> {
    let mut providers = list_providers()?;
    providers.retain(|p| p.id != id);
    persist(&providers)
}

/// 测试连接：对配置的端点做一次最小请求
pub fn test_provider(provider: &AIProviderConfig) -> Result<bool, String> {
    if provider.encrypted_api_key.starts_with("dpapi:") {
        let _ = dpapi_decrypt(provider.encrypted_api_key.trim_start_matches("dpapi:"))?;
    }
    // 简单验证：OpenAI 兼容端点返回模型列表
    let url = format!("{}/models", provider.api_base_url.trim_end_matches('/'));
    let api_key = provider.encrypted_api_key.trim_start_matches("dpapi:");
    let resp = ureq::get(&url)
        .set("Authorization", &format!("Bearer {api_key}"))
        .timeout(std::time::Duration::from_secs(8))
        .call();
    match resp {
        Ok(_) => Ok(true),
        Err(ureq::Error::Status(_, _)) => Ok(true), // 端点可达（可能需更高权限）
        Err(e) => Err(format!("连接失败: {e}")),
    }
}

fn persist(providers: &[AIProviderConfig]) -> Result<(), String> {
    let path = providers_file();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(providers).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

// ---- 系统上下文与分析 ----

/// 构建系统上下文文本（对齐 C# SystemContextBuilder 的简化版）
pub fn build_system_context(system_info: Value) -> String {
    let drive = system_info
        .get("systemDrive")
        .and_then(Value::as_str)
        .unwrap_or("C:");
    let free_gb = system_info
        .get("systemDriveFreeBytes")
        .and_then(Value::as_u64)
        .map(|b| b as f64 / 1024.0 / 1024.0 / 1024.0)
        .unwrap_or(0.0);
    let total_gb = system_info
        .get("systemDriveTotalBytes")
        .and_then(Value::as_u64)
        .map(|b| b as f64 / 1024.0 / 1024.0 / 1024.0)
        .unwrap_or(0.0);
    let os = system_info
        .get("osVersion")
        .and_then(Value::as_str)
        .unwrap_or("Unknown");
    let cores = system_info
        .get("processorCount")
        .and_then(Value::as_u64)
        .unwrap_or(0);

    format!(
        "=== AetherOS 系统信息 ===\n\
         操作系统: {os}\n\
         CPU 核心数: {cores}\n\
         系统盘 {drive}: 总容量 {total_gb:.1} GB, 可用 {free_gb:.1} GB (占用 {:.0}%)\n\
         \n请以 JSON 格式输出系统优化建议：{{ summary, riskLevel, urgency, sections: [{{ module, title, analysis, recommendations: [{{ action, reason, impact, risk, priority }}] }}] }}",
        (1.0 - free_gb / total_gb.max(1.0)) * 100.0
    )
}

/// 调用 LLM 执行分析（OpenAI 兼容 + Anthropic）
pub fn run_analysis(
    provider: &AIProviderConfig,
    context: &str,
) -> Result<AIAnalysisResult, String> {
    let api_key = if provider.encrypted_api_key.starts_with("dpapi:") {
        dpapi_decrypt(provider.encrypted_api_key.trim_start_matches("dpapi:"))?
    } else {
        provider.encrypted_api_key.clone()
    };

    let result = match provider.provider_type {
        AIProviderType::OpenAICompatible => call_openai(provider, &api_key, context)?,
        AIProviderType::AnthropicClaude => call_anthropic(provider, &api_key, context)?,
    };

    Ok(result)
}

fn call_openai(
    provider: &AIProviderConfig,
    api_key: &str,
    context: &str,
) -> Result<AIAnalysisResult, String> {
    let url = format!(
        "{}/chat/completions",
        provider.api_base_url.trim_end_matches('/')
    );
    let body = json!({
        "model": provider.model_id,
        "messages": [
            { "role": "system", "content": "你是 AetherOS 系统优化助手，只输出 JSON。" },
            { "role": "user", "content": context }
        ],
        "temperature": 0.3,
        "response_format": { "type": "json_object" }
    });
    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {api_key}"))
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(60))
        .send_json(body)
        .map_err(|e| format!("请求失败: {e}"))?;
    let value: Value = resp.into_json().map_err(|e| format!("响应解析失败: {e}"))?;
    let content = value
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .ok_or_else(|| "响应缺少 content".to_string())?;
    parse_analysis(content, provider)
}

fn call_anthropic(
    provider: &AIProviderConfig,
    api_key: &str,
    context: &str,
) -> Result<AIAnalysisResult, String> {
    let url = format!(
        "{}/v1/messages",
        provider.api_base_url.trim_end_matches('/')
    );
    let body = json!({
        "model": provider.model_id,
        "max_tokens": 2048,
        "system": "你是 AetherOS 系统优化助手，只输出 JSON。",
        "messages": [{ "role": "user", "content": context }]
    });
    let resp = ureq::post(&url)
        .set("x-api-key", api_key)
        .set("anthropic-version", "2023-06-01")
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(60))
        .send_json(body)
        .map_err(|e| format!("请求失败: {e}"))?;
    let value: Value = resp.into_json().map_err(|e| format!("响应解析失败: {e}"))?;
    let content = value
        .pointer("/content/0/text")
        .and_then(Value::as_str)
        .ok_or_else(|| "响应缺少 text".to_string())?;
    parse_analysis(content, provider)
}

/// 解析 LLM 返回的 JSON 为 AIAnalysisResult
fn parse_analysis(content: &str, provider: &AIProviderConfig) -> Result<AIAnalysisResult, String> {
    // 提取 JSON（可能包裹在 ```json 代码块中）
    let trimmed = content.trim();
    let json_start = trimmed.find('{').unwrap_or(0);
    let json_end = trimmed.rfind('}').map(|i| i + 1).unwrap_or(trimmed.len());
    let json_str = &trimmed[json_start..json_end];

    let value: Value =
        serde_json::from_str(json_str).map_err(|e| format!("分析结果 JSON 解析失败: {e}"))?;
    let generated_at = chrono::Utc::now();

    let section_to_ai = |s: &Value| -> aether_core::models::ai::AIAnalysisSection {
        let recommendations = s
            .get("recommendations")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .map(|r| aether_core::models::ai::AIRecommendation {
                        action: r
                            .get("action")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                        reason: r
                            .get("reason")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                        impact: r
                            .get("impact")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                        risk: r
                            .get("risk")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                        priority: r
                            .get("priority")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                        is_selected: true,
                    })
                    .collect()
            })
            .unwrap_or_default();
        aether_core::models::ai::AIAnalysisSection {
            module: s
                .get("module")
                .and_then(Value::as_str)
                .unwrap_or("config")
                .to_string(),
            title: s
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            analysis: s
                .get("analysis")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            recommendations,
        }
    };

    let sections = value
        .get("sections")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().map(section_to_ai).collect())
        .unwrap_or_default();

    Ok(AIAnalysisResult {
        summary: value
            .get("summary")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        risk_level: value
            .get("riskLevel")
            .and_then(Value::as_str)
            .unwrap_or("low")
            .to_string(),
        urgency: value
            .get("urgency")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        sections,
        generated_at,
        provider_name: provider.name.clone(),
        model_id: provider.model_id.clone(),
        is_successful: true,
        error_message: None,
    })
}
