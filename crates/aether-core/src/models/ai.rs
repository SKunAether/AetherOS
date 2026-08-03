//! AI 分析模型（对齐 C# AIProviderConfig / AIAnalysisResult）

use serde::{Deserialize, Serialize};

/// AI 服务商类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AIProviderType {
    /// OpenAI 兼容接口
    OpenAICompatible,
    /// Anthropic Claude
    AnthropicClaude,
}

/// AI 服务商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: AIProviderType,
    pub api_base_url: String,
    /// DPAPI 加密后的 API Key
    pub encrypted_api_key: String,
    pub model_id: String,
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(default)]
    pub is_default: bool,
}

/// AI 分析建议
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIRecommendation {
    pub action: String,
    pub reason: String,
    pub impact: String,
    pub risk: String,
    pub priority: String,
    #[serde(default)]
    pub is_selected: bool,
}

/// AI 分析章节
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIAnalysisSection {
    /// disk_space / hibernate / startup / cleanup_plan / recovery / config
    pub module: String,
    pub title: String,
    pub analysis: String,
    pub recommendations: Vec<AIRecommendation>,
}

/// AI 分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIAnalysisResult {
    pub summary: String,
    pub risk_level: String,
    pub urgency: String,
    pub sections: Vec<AIAnalysisSection>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub provider_name: String,
    pub model_id: String,
    pub is_successful: bool,
    pub error_message: Option<String>,
}
