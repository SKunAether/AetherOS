# AetherOS 2.0 Architecture

## Overview

AetherOS 2.0 is a modular Windows system optimization & management platform built on
**Rust + Tauri + React + TypeScript + TailwindCSS**. The architecture is a classic
layered design with one-way dependencies top-down; inner layers never depend on outer ones.

```
┌──────────────────────────────────────────────────────┐
│                    React UI Layer                     │
│   Dashboard / Feature pages / Components / Router     │
└──────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────┐
│                  Tauri IPC Bridge                     │
│   Command invocation / Event subscription / Serialize │
└──────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────┐
│                 Command & Service Layer              │
│   Command entrypoints / Service orchestration        │
└──────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────┐
│               Capability Runtime Layer               │
│   Provider registry / Lifecycle / Plugin loading /   │
│   Event bus                                          │
└──────────────────────────────────────────────────────┘
┌──────────┬──────────┬──────────┬─────────────────────┐
│ Cleaner  │Optimizer │ Monitor  │     Recovery        │
│ Provider │ Provider │ Provider │     Provider        │
└──────────┴──────────┴──────────┴─────────────────────┘
┌──────────────────────────────────────────────────────┐
│                  System Engine Layer                 │
│   Windows API wrappers (registry/process/service/    │
│   filesystem/hardware)                               │
└──────────────────────────────────────────────────────┘
```

## Design Principles

- **Capability Provider model**: every system capability is exposed through the
  standardized `CapabilityProvider` trait — a uniform abstraction, capability
  declaration, unified scheduling, and plugin extensibility.
- **Layered isolation**: the React UI talks to Rust only through Tauri IPC; providers
  never call Windows APIs directly (all platform calls are confined to `aether-system`),
  which keeps the platform layer swappable for future macOS/Linux support.
- **Frontend/backend separation**: IPC calls are wrapped in the frontend `api/` layer and
  cached with TanStack Query; state is managed with Zustand.
- **Feature-Sliced frontend**: code is organized by business feature, not by file type.

## Modules

| Crate / package | Responsibility |
|---|---|
| `aether-core` | Domain models, `CapabilityProvider` trait, errors, events. No platform dependency. |
| `aether-runtime` | Provider registry, task scheduler, plugin loader, event bus. |
| `aether-system` | Windows API wrappers: registry, process, service, filesystem, hardware. |
| `aether-cleaner` | Cleanup capability: temp files, caches, logs, recycle bin, registry junk. |
| `aether-optimizer` | Optimization: power plans, startup items, services, network, visual effects. |
| `aether-monitor` | Real-time CPU/memory/disk/network monitoring and health scoring. |
| `aether-recovery` | Restore points, config backup & restore, operation rollback. |
| `aether-plugin-sdk` | Plugin development SDK for third-party capability extensions. |
| `apps/desktop` | Tauri 2 desktop shell (Rust) + React frontend. |

## Data Flow

1. React components call wrapped functions in `api/` → Tauri `invoke`.
2. Tauri commands dispatch to the capability runtime or directly to providers.
3. Providers use `aether-system` services to perform actual system operations.
4. Long-running tasks publish progress via the event bus → Tauri events → React listeners.

## Migration Notes

The v1 codebase (C#/WPF under `src/`) is retained as history. Business logic and rule
tables from the old services (scanning/cleanup/power plans/history) were ported to the
new Rust providers; the plugin system evolved from declarative `plugin.json` manifests
to real `CapabilityProvider` implementations.
