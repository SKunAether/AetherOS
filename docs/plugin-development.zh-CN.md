# AetherOS 插件开发文档

> 适用于 AetherOS 2.x（Rust + Tauri + React）。

## 概述

AetherOS 的能力通过 `CapabilityProvider` trait 暴露。插件以两种方式扩展平台：

1. **声明式规则包** —— 含 `plugin.json` 清单的 ZIP 包（同 v1）。
   不携带可执行代码；声明 `scan.temp`、`scan.thumbnail` 等能力。
2. **代码插件** —— 通过 `aether-plugin-sdk` crate 实现 `CapabilityProvider` trait，
   由 `aether-runtime` 动态加载。

## 插件清单（`plugin.json`）

```json
{
  "id": "com.example.cleaner",
  "name": "示例清理插件",
  "version": "1.0.0",
  "author": "AetherOS Community",
  "description": "示例能力插件",
  "category": "Rule Pack",
  "minimumAetherOSVersion": "2.0.0",
  "capabilities": ["scan.temp", "scan.thumbnail"],
  "isEnabled": false,
  "isBuiltIn": false
}
```

## 代码插件（SDK）

实现 `CapabilityProvider`：

```rust
use aether_plugin_sdk::prelude::*;

struct MyProvider;

#[async_trait]
impl CapabilityProvider for MyProvider {
    fn id(&self) -> &str { "com.example.my-provider" }
    fn name(&self) -> &str { "我的 Provider" }
    fn capability_type(&self) -> CapabilityType { CapabilityType::Cleaner }

    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError> {
        Ok(json!({ "hello": action }))
    }

    async fn scan(&self, params: Value) -> Result<ScanResult, CapabilityError> {
        // ...
    }
}
```

## 打包与分发

- 声明式规则包：将 `plugin.json`（+ 可选规则资源）打包为 ZIP，通过插件中心安装。
- 代码插件：编译为共享库并在运行时注册（Phase 6 落地）。

> 安全说明：v1/v2 声明式规则包绝不执行第三方任意代码；
> 代码插件需沙箱隔离并经用户明确授权。
