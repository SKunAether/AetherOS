# AetherOS Plugin Development

> Applies to AetherOS 2.x (Rust + Tauri + React).

## Overview

AetherOS capabilities are exposed through the `CapabilityProvider` trait. Plugins extend
the platform in two ways:

1. **Declarative rule packs** — a `plugin.json` manifest packaged as a ZIP (like v1).
   No executable code; declares capabilities such as `scan.temp`, `scan.thumbnail`.
2. **Code plugins** — implement the `CapabilityProvider` trait via the
   `aether-plugin-sdk` crate and are dynamically loaded by `aether-runtime`.

## Plugin Manifest (`plugin.json`)

```json
{
  "id": "com.example.cleaner",
  "name": "Example Cleaner",
  "version": "1.0.0",
  "author": "AetherOS Community",
  "description": "Example capability plugin",
  "category": "Rule Pack",
  "minimumAetherOSVersion": "2.0.0",
  "capabilities": ["scan.temp", "scan.thumbnail"],
  "isEnabled": false,
  "isBuiltIn": false
}
```

## Code Plugin (SDK)

Implement `CapabilityProvider`:

```rust
use aether_plugin_sdk::prelude::*;

struct MyProvider;

#[async_trait]
impl CapabilityProvider for MyProvider {
    fn id(&self) -> &str { "com.example.my-provider" }
    fn name(&self) -> &str { "My Provider" }
    fn capability_type(&self) -> CapabilityType { CapabilityType::Cleaner }

    async fn execute(&self, action: &str, params: Value) -> Result<Value, CapabilityError> {
        Ok(json!({ "hello": action }))
    }

    async fn scan(&self, params: Value) -> Result<ScanResult, CapabilityError> {
        // ...
    }
}
```

## Packaging & Distribution

- Declarative packs: ZIP the `plugin.json` (+ optional rule assets), install via the
  plugin center.
- Code plugins: compile to a shared library and register with the runtime (Phase 6).

> Security note: v1/v2 declarative packs never execute arbitrary third-party code.
> Code plugins are sandboxed and require explicit user approval.
