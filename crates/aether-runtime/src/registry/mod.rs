//! Provider 注册中心：扫描、注册、查询所有能力提供者
//!
//! 运行时启动时自动扫描并注册所有内置与外部 Provider，
//! 通过 `CapabilityProvider` trait 对象统一管理。

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use aether_core::traits::{ActionDef, CapabilityProvider, CapabilityType};
use serde::{Deserialize, Serialize};

/// Provider 描述符（对前端展示用）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDescriptor {
    pub id: String,
    pub name: String,
    pub description: String,
    pub capability_type: CapabilityType,
    pub actions: Vec<ActionDef>,
    #[serde(default)]
    pub is_builtin: bool,
}

/// Provider 注册中心
///
/// 线程安全（RwLock 内部可变性），允许多个 Provider 并发注册与查询。
#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: Arc<RwLock<HashMap<String, Arc<dyn CapabilityProvider>>>>,
}

impl ProviderRegistry {
    /// 创建空的注册中心
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册一个 Provider（按 id 去重，后注册的覆盖先注册的）
    pub fn register<P: CapabilityProvider + 'static>(&self, provider: P) {
        let id = provider.id().to_string();
        let mut map = self.providers.write().unwrap();
        map.insert(id, Arc::new(provider));
    }

    /// 注册一个已装箱的 Provider
    pub fn register_boxed(&self, provider: Arc<dyn CapabilityProvider>) {
        let id = provider.id().to_string();
        let mut map = self.providers.write().unwrap();
        map.insert(id, provider);
    }

    /// 注销一个 Provider
    pub fn unregister(&self, id: &str) -> Option<Arc<dyn CapabilityProvider>> {
        self.providers.write().unwrap().remove(id)
    }

    /// 按 id 查询 Provider
    pub fn get(&self, id: &str) -> Option<Arc<dyn CapabilityProvider>> {
        self.providers.read().unwrap().get(id).cloned()
    }

    /// 列出所有 Provider id
    pub fn ids(&self) -> Vec<String> {
        self.providers.read().unwrap().keys().cloned().collect()
    }

    /// 按能力类型筛选 Provider id
    pub fn by_capability(&self, capability: CapabilityType) -> Vec<String> {
        self.providers
            .read()
            .unwrap()
            .values()
            .filter(|p| p.capability_type() == capability)
            .map(|p| p.id().to_string())
            .collect()
    }

    /// 已注册 Provider 数量
    pub fn len(&self) -> usize {
        self.providers.read().unwrap().len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.providers.read().unwrap().is_empty()
    }

    /// 列出全部 Provider 描述符（前端渲染用）
    pub fn descriptors(&self) -> Vec<ProviderDescriptor> {
        self.providers
            .read()
            .unwrap()
            .values()
            .map(|p| ProviderDescriptor {
                id: p.id().to_string(),
                name: p.name().to_string(),
                description: p.description().to_string(),
                capability_type: p.capability_type(),
                actions: p.actions(),
                is_builtin: true,
            })
            .collect()
    }
}
