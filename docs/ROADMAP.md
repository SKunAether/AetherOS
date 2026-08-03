# AetherOS Roadmap

> AetherOS 2.0 is a complete rewrite of the v1 C#/WPF codebase onto a Rust + Tauri + React stack.

## v2.0 — Architecture Rewrite (Rust + Tauri + React)

Migration from C#/WPF to the modular Capability-Provider platform, executed in phases.

| Phase | Scope | Status |
|---|---|---|
| **Phase 0** | Project skeleton: Cargo workspace, pnpm monorepo, Tauri shell, React framework, Tailwind theme, i18n, CI | ✅ Done |
| **Phase 1** | Core runtime: `aether-core` models/trait, `aether-runtime` registry + event bus + scheduler, IPC command layer, demo provider | ✅ Done |
| **Phase 2** | Dashboard & UI framework: sidebar/topbar layout, routing, theme toggle, bilingual switch, basic IPC, real system-info dashboard | ✅ Done |
| **Phase 3** | System info & monitoring: `aether-system` hardware, `aether-monitor` provider, live charts | ✅ Done |
| **Phase 4** | Cleanup module: scan rules, execute pipeline, progress & results (full chain) | ✅ Done |
| **Phase 5** | Optimizer & recovery: power plans, startup items, virtual memory, restore points, action history | ✅ Done |
| **Phase 6** | Plugin SDK & engineering: SDK, tray/startup/silent, AI analysis, release workflows (dynamic loading deferred) | ✅ Done |

## v2.x

- Improve plugin ecosystem (builtin + example rule packs)
- Expand automated testing (Rust unit + frontend component + E2E)
- Improve bilingual documentation

## Future

- More platform abstraction (macOS / Linux system engines)
- Enhanced diagnostics and telemetry
- Community extension marketplace
