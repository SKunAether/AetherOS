# AetherOS Guardian

**Pure-Local Windows System Inspection, Cleanup, Configuration, and Recovery Tool**

[![Build Status](https://github.com/SKunAether/AetherOS/actions/workflows/build.yml/badge.svg)](https://github.com/SKunAether/AetherOS/actions/workflows/build.yml)
[![Release](https://img.shields.io/github/v/release/SKunAether/AetherOS)](https://github.com/SKunAether/AetherOS/releases)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Tech](https://img.shields.io/badge/Stack-Rust%20%7C%20Tauri%202%20%7C%20React%2019-blue)](Cargo.toml)

English | [**简体中文**](README.zh-CN.md)

---

> ✅ **v2.0 architecture rewrite complete**: AetherOS has migrated from .NET 8 / WPF to a
> **Rust + Tauri 2 + React 19** stack with a modular Capability-Provider platform architecture.
> See progress at [docs/ROADMAP.md](docs/ROADMAP.md).

---

## 📋 Introduction

**AetherOS Guardian** is a modular Windows system optimization & management platform that provides comprehensive system maintenance capabilities. It features both **rule-based local analysis** (default) and **optional cloud AI analysis** (via multiple AI providers).

Unlike traditional optimization tools, AetherOS Guardian emphasizes **explainable rules**, **user-controlled execution**, and **full auditability**. All system changes require explicit user confirmation, and supported actions are recorded for recovery and rollback.

### Key Highlights
- **Local-First**: Core functionality works offline. AI analysis is opt-in and disabled by default.
- **User Control**: Every system change requires approval.
- **Auditable & Recoverable**: Full operation history with one-click rollback.
- **Optional AI Analysis**: Multi-provider support (OpenAI-compatible, Anthropic Claude) — disabled by default, data sent only to your configured provider.
- **Open Source**: Transparent, customizable, and community-driven.

---

## ✨ Features

| Feature | Description |
| :--- | :--- |
| 🖥️ **System Overview** | Dashboard displaying system health, disk usage, and quick status. |
| 🔍 **Local System Inspection** | Rule-based inspection with direct links to handling modules. |
| 📦 **Storage Analysis** | Identify and clean up temporary files, caches, and large unused data. |
| ⚡ **System Tuning** | Manage power plans, hibernation, virtual memory, and startup items. |
| 🔄 **Recovery Center** | Full operation history with one-click rollback for supported actions. |
| 🧩 **Rule Extensions** | Extend functionality via local rule packs (no arbitrary DLL execution). |
| 🛠️ **System Tray** | Minimize to tray, quick access, and scheduled background scans. |
| 🤖 **AI Analysis** (optional) | Cloud AI analysis for scan results with multi-provider switching (OpenAI, DeepSeek, Claude, etc.) |
| 🔐 **Privacy & Security** | Built-in privacy scanner prevents accidental leakage of personal data. AI analysis is opt-in. |
| 📥 **Auto-Update** | GitHub Releases based update checker with SHA-256 verification. |

---

## 🚀 Quick Start

### For End Users (Download Installer)
1. Go to the [Releases](https://github.com/SKunAether/AetherOS/releases) page.
2. Download the latest `AetherOS-Guardian-{version}-win-x64.zip`.
3. Extract the archive and run `AetherOS.Guardian.exe`.

> **Note:** Right-click and select **"Run as administrator"** to enable full functionality (power plan changes, system restore, etc.).

### For Developers (Build from Source)

#### Prerequisites
- Windows 10 / Windows 11
- [Rust 1.80+](https://www.rust-lang.org/) (with MSVC linker)
- [Node.js 20+](https://nodejs.org/) and [pnpm](https://pnpm.io/)
- WebView2 (bundled on Windows 11; install on Windows 10)

#### Install dependencies
```bash
# Frontend dependencies (includes @tauri-apps/cli)
cd apps/desktop/frontend
pnpm install
```

#### Run in development
```bash
cd apps/desktop/frontend
pnpm tauri dev
```

#### Build the installer
```bash
cd apps/desktop/frontend
pnpm tauri build
# Output: apps/desktop/src-tauri/target/release/bundle/nsis/*.exe
```

#### Create a Local Release Package

powershell

```
.\scripts\Release.ps1 -GitHubOwner "SKunAether" -GitHubRepository "AetherOS"
```

Generated assets will be located in `artifacts/release/`.

------

## 📂 Project Structure

```
AetherOS/
├── .cargo/                           # Rust compiler configuration
│   └── config.toml                   # Build target, linker settings, etc.
├── .github/                          # GitHub automation
│   ├── ISSUE_TEMPLATE/               # Issue templates
│   │   ├── bug_report.md             # Template for bug reports
│   │   └── feature_request.md        # Template for feature requests
│   ├── workflows/                    # CI/CD workflows
│   │   ├── build.yml                 # Build check on push/PR
│   │   ├── nightly.yml               # Optional nightly builds
│   │   └── release.yml               # Tag-triggered release (build installer, update manifest)
│   ├── dependabot.yml                # Automated dependency updates
│   ├── FUNDING.yml                   # Sponsorship information
│   └── PULL_REQUEST_TEMPLATE.md      # PR description template
├── apps/                             # Application collection
│   └── desktop/                      # Desktop application (main program)
│       ├── frontend/                 # React frontend (TypeScript + Tailwind CSS)
│       │   ├── dist/                 # Frontend build output (Vite)
│       │   │   ├── assets/           # Compiled JS/CSS files
│       │   │   │   ├── index-BuvkXy0c.js
│       │   │   │   ├── index-DVuYSGJf.css
│       │   │   │   └── window-CVrULaN6.js
│       │   │   └── index.html        # Entry HTML
│       │   ├── public/               # Static assets (icons, manifest, etc.)
│       │   ├── src/                  # Frontend source code
│       │   │   ├── api/              # API wrappers for Tauri backend
│       │   │   │   ├── ai.ts         # AI analysis interface
│       │   │   │   ├── invoke.ts     # Tauri invoke wrapper
│       │   │   │   ├── providers.ts  # Capability provider (plugin) API
│       │   │   │   ├── runtime.ts    # Runtime information
│       │   │   │   ├── settings.ts   # Settings read/write
│       │   │   │   └── system.ts     # System info (hardware, OS, etc.)
│       │   │   ├── app/              # Global layout and entry
│       │   │   │   ├── layouts/
│       │   │   │   │   └── AppLayout.tsx   # Main layout (sidebar + content)
│       │   │   │   └── App.tsx       # Root component (routing, theme)
│       │   │   ├── components/       # Reusable UI components
│       │   │   │   ├── charts/
│       │   │   │   │   └── Sparkline.tsx  # Mini trend chart
│       │   │   │   └── ui/           # Basic components (buttons, cards, etc.)
│       │   │   │       ├── button.tsx
│       │   │   │       ├── card.tsx
│       │   │   │       ├── placeholder.tsx
│       │   │   │       └── progress.tsx
│       │   │   ├── features/         # Feature pages (business modules)
│       │   │   │   ├── cleaner/      # Cleaning & optimization
│       │   │   │   │   └── CleanerPage.tsx
│       │   │   │   ├── dashboard/    # System overview dashboard
│       │   │   │   │   └── DashboardPage.tsx
│       │   │   │   ├── monitor/      # System monitoring (resource usage)
│       │   │   │   │   └── MonitorPage.tsx
│       │   │   │   ├── optimizer/    # System tuning (startup, power, etc.)
│       │   │   │   │   └── OptimizerPage.tsx
│       │   │   │   ├── recovery/     # Recovery center (history, rollback)
│       │   │   │   │   └── RecoveryPage.tsx
│       │   │   │   └── settings/     # Settings (including AI configuration)
│       │   │   │       ├── AboutPage.tsx
│       │   │   │       └── SettingsPage.tsx
│       │   │   ├── hooks/            # Custom React Hooks
│       │   │   │   ├── use-backend-event.ts  # Listen to backend events
│       │   │   │   ├── use-interval.ts        # Polling timer
│       │   │   │   ├── use-providers.ts       # Fetch available capability providers
│       │   │   │   ├── use-settings.ts        # Read/write settings
│       │   │   │   └── use-system-info.ts     # Fetch system information
│       │   │   ├── i18n/             # Internationalization setup
│       │   │   │   └── index.ts      # i18n initialization
│       │   │   ├── locales/          # Language files (JSON)
│       │   │   │   ├── en-US/        # English
│       │   │   │   │   ├── common.json
│       │   │   │   │   ├── dashboard.json
│       │   │   │   │   └── nav.json
│       │   │   │   └── zh-CN/        # Simplified Chinese
│       │   │   │       ├── common.json
│       │   │   │       ├── dashboard.json
│       │   │   │       └── nav.json
│       │   │   ├── stores/           # State management (Zustand)
│       │   │   │   └── app-store.ts  # Global application state
│       │   │   ├── styles/           # Global styles (Tailwind entry)
│       │   │   │   └── globals.css
│       │   │   ├── types/            # TypeScript type declarations
│       │   │   │   └── index.ts      # Shared types
│       │   │   ├── main.tsx          # Frontend entry point
│       │   │   └── vite-env.d.ts     # Vite environment types
│       │   ├── .prettierignore       # Prettier ignore list
│       │   ├── .prettierrc.json      # Code formatting config
│       │   ├── eslint.config.js      # ESLint rules
│       │   ├── index.html            # HTML template
│       │   ├── package.json          # Frontend dependencies
│       │   ├── postcss.config.js     # PostCSS config (Tailwind)
│       │   ├── tailwind.config.js    # Tailwind CSS config
│       │   ├── tsconfig.json         # TypeScript compiler options
│       │   ├── tsconfig.node.json    # TypeScript for Node environment
│       │   └── vite.config.ts        # Vite build configuration
│       └── src-tauri/                # Tauri backend (Rust)
│           ├── capabilities/         # Capability declarations (permissions)
│           │   └── default.json      # Default allowed permissions
│           ├── gen/                  # Auto-generated schemas (for frontend)
│           │   └── schemas/
│           │       ├── acl-manifests.json
│           │       ├── capabilities.json
│           │       ├── desktop-schema.json
│           │       └── windows-schema.json
│           ├── icons/                # Application icons
│           │   └── icon.ico          # Windows icon
│           ├── src/                  # Rust source code
│           │   ├── ai/               # AI analysis module
│           │   │   └── mod.rs        # AI service invocation logic
│           │   ├── commands/         # Tauri commands (exposed to frontend)
│           │   │   ├── ai.rs         # AI analysis command
│           │   │   ├── cleaner.rs    # Cleaning operations
│           │   │   ├── hibernate.rs  # Hibernate management
│           │   │   ├── mod.rs        # Command registration entry
│           │   │   ├── power.rs      # Power plan management
│           │   │   ├── quick_scan.rs # Quick scan
│           │   │   ├── runtime.rs    # Runtime information
│           │   │   ├── settings.rs   # Settings read/write
│           │   │   └── system.rs     # System information
│           │   ├── providers/        # Capability provider (plugin) examples
│           │   │   ├── demo.rs       # Demo provider
│           │   │   └── mod.rs        # Provider registration
│           │   ├── error.rs          # Unified error types
│           │   ├── lib.rs            # Library entry (for other crates)
│           │   ├── main.rs           # Main entry (starts Tauri)
│           │   ├── settings.rs       # Settings management (reads tauri.conf.json)
│           │   ├── startup.rs        # Startup item management
│           │   ├── state.rs          # Global application state (e.g., config)
│           │   └── tray.rs           # System tray menu and events
│           ├── build.rs              # Build script (generates resources)
│           ├── Cargo.toml            # Backend dependencies
│           └── tauri.conf.json       # Tauri core config (app name, windows, permissions)
├── assets/                           # Static resources (images, icons, donation QR codes)
│   ├── icons/
│   │   ├── AetherOS.ico              # Application icon (multi-size)
│   │   └── AetherOS.png              # PNG version of icon
│   ├── alipay_pay.png                # Alipay donation QR
│   └── wechat_pay.png                # WeChat donation QR
├── crates/                           # Rust workspace members (modular capabilities)
│   ├── aether-cleaner/               # Cleaning capability (temp files, caches, etc.)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── provider.rs           # Implements CapabilityProvider trait
│   │   └── Cargo.toml
│   ├── aether-core/                  # Core data models and traits (shared across crates)
│   │   ├── src/
│   │   │   ├── errors/               # Error types (capability-related)
│   │   │   │   ├── capability.rs
│   │   │   │   └── mod.rs
│   │   │   ├── events/               # Event bus types
│   │   │   │   └── mod.rs
│   │   │   ├── models/               # Business models (scan results, operation records, settings, etc.)
│   │   │   │   ├── action.rs         # Action definition (executable operation)
│   │   │   │   ├── ai.rs             # AI analysis result
│   │   │   │   ├── cleaner.rs        # Cleanable item
│   │   │   │   ├── deep_scan.rs      # Deep scan result
│   │   │   │   ├── diagnostics.rs    # Diagnostic info
│   │   │   │   ├── hibernate.rs      # Hibernate state
│   │   │   │   ├── maintenance.rs    # Maintenance task
│   │   │   │   ├── mod.rs
│   │   │   │   ├── operation.rs      # Operation history record
│   │   │   │   ├── pagefile.rs       # Virtual memory config
│   │   │   │   ├── plugin.rs         # Plugin descriptor
│   │   │   │   ├── power.rs          # Power plan
│   │   │   │   ├── quick_scan.rs     # Quick scan result
│   │   │   │   ├── restore_point.rs  # System restore point
│   │   │   │   ├── settings.rs       # Application settings
│   │   │   │   └── system_context.rs # System context (snapshot)
│   │   │   ├── traits/               # Capability provider interface
│   │   │   │   ├── capability_provider.rs
│   │   │   │   └── mod.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── aether-monitor/               # System monitoring capability (CPU/memory/disk)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── provider.rs
│   │   └── Cargo.toml
│   ├── aether-optimizer/             # System optimization capability (startup, service tuning)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── provider.rs
│   │   │   └── startup.rs            # Startup item management implementation
│   │   └── Cargo.toml
│   ├── aether-plugin-sdk/            # Plugin development SDK (for third-party plugins)
│   │   ├── src/
│   │   │   └── lib.rs                # Exports macros and types needed by plugins
│   │   └── Cargo.toml
│   ├── aether-recovery/              # Recovery capability (operation history, rollback)
│   │   ├── src/
│   │   │   ├── history.rs            # History storage and query
│   │   │   ├── lib.rs
│   │   │   └── provider.rs
│   │   └── Cargo.toml
│   ├── aether-runtime/               # Runtime core (plugin management, event bus, scheduling)
│   │   ├── src/
│   │   │   ├── bus/                  # Event bus (inter-module communication)
│   │   │   │   └── mod.rs
│   │   │   ├── plugin/               # Plugin loading and lifecycle
│   │   │   │   └── mod.rs
│   │   │   ├── registry/             # Capability registry
│   │   │   │   └── mod.rs
│   │   │   ├── scheduler/            # Background task scheduling (scheduled scans, etc.)
│   │   │   │   └── mod.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── aether-system/                # System information retrieval (hardware, OS, registry, services)
│       ├── src/
│       │   ├── filesystem/           # Filesystem operations
│       │   │   └── mod.rs
│       │   ├── hardware/             # Hardware info (CPU, memory, etc.)
│       │   │   └── mod.rs
│       │   ├── power/                # Power management (API calls)
│       │   │   └── mod.rs
│       │   ├── process/              # Process management
│       │   │   └── mod.rs
│       │   ├── registry/             # Windows registry read/write
│       │   │   └── mod.rs
│       │   ├── service/              # Windows service management
│       │   │   └── mod.rs
│       │   └── lib.rs
│       └── Cargo.toml
├── docs/                             # Extended documentation
│   ├── ARCHITECTURE.md               # Architecture design (English)
│   ├── ARCHITECTURE.zh-CN.md         # Architecture design (Chinese)
│   ├── plugin-development.md         # Plugin development guide (English)
│   ├── plugin-development.zh-CN.md   # Plugin development guide (Chinese)
│   ├── PROJECT-STRUCTURE.md          # Project structure description (English)
│   ├── RELEASE-GUIDE.md              # Release process guide
│   ├── ROADMAP.md                    # Development roadmap (English)
│   └── ROADMAP.zh-CN.md              # Development roadmap (Chinese)
├── installer/                        # Installer-related files
│   ├── scripts/                      # Installation scripts (for future use)
│   ├── windows/                      # Windows-specific installation config
│   │   └── README.md                 # Installation instructions
│   └── update-manifest.template.json # Update manifest template (for auto-updates)
├── plugins/                          # Plugin definitions (declarative configuration)
│   ├── builtin/                      # Built-in plugins (shipped with the main app)
│   │   ├── developer-environment/    # Developer environment detection
│   │   │   └── plugin.json
│   │   ├── storage-analysis/         # Storage analysis
│   │   │   └── plugin.json
│   │   └── windows-cleaner/          # Windows standard cleanup
│   │       └── plugin.json
│   └── examples/                     # Example plugins (for developer reference)
│       └── sample-rule-pack/
│           ├── plugin.json           # Plugin metadata (name, version, capability declaration)
│           └── README.md             # Example description
├── .editorconfig                     # Editor unified configuration
├── .env.example                      # Environment variables example (CI, etc.)
├── .gitignore                        # Git ignore rules (includes target, node_modules, etc.)
├── Cargo.lock                        # Rust dependency lock file
├── Cargo.toml                       # Rust workspace root manifest (defines all sub-crates)
├── CHANGELOG.md                      # Changelog (English)
├── CHANGELOG.zh-CN.md                # Changelog (Chinese)
├── CODE_OF_CONDUCT.md                # Code of conduct
├── CONTRIBUTING.md                   # Contribution guidelines
├── LICENSE                           # AGPL-3.0 license
├── package.json                      # pnpm workspace root (for frontend management)
├── pnpm-lock.yaml                    # pnpm lock file
├── pnpm-workspace.yaml               # pnpm workspace config (points to apps/desktop/frontend)
├── README.md                         # Project README (English)
├── README.zh-CN.md                   # Project README (Chinese)
├── rust-toolchain.toml               # Specifies Rust toolchain version
└── SECURITY.md                       # Security policy (vulnerability reporting)
```



------

## ❓ FAQ

**Q: Does the application require an internet connection?**
A: Core features (system inspection, cleaning, configuration) work entirely offline. No telemetry is collected. If you choose to enable "AI Analysis" in Settings, system scan data will be sent to your configured AI provider. AI analysis is **disabled by default** and must be explicitly enabled.

**Q: Why does it need administrator privileges?**
A: Some features require elevated privileges to work properly, including:

- Switching power plans
- Enabling/disabling hibernation
- Creating system restore points
- Accessing system-protected directories
- Modifying virtual memory settings

If you only use read-only features like System Overview and Storage Analysis, you can run without administrator privileges, but functionality will be limited.

**Q: Does the program contain AI features?**
A: AetherOS Guardian offers optional cloud AI analysis that can provide intelligent recommendations based on scan results. This feature supports multiple providers (OpenAI, DeepSeek, Claude, Qwen, and any OpenAI-compatible API). It is **disabled by default** — you must explicitly enable it in Settings and configure your own API key. API keys are encrypted using Windows DPAPI and stored locally. When enabled, system scan data (disk usage, file sizes — not personal file contents) is sent to your configured AI provider.

**Q: Does the program run continuously in the background?**
A: Not by default. The program only runs when you actively open it. If you enable "Close to tray," it will stay in the system tray. If you enable "Scheduled scans," it will run background scans at the scheduled times. Other than these, no background activity occurs without user action.

**Q: How much system resources does scanning and cleaning consume?**
A: CPU and disk usage will temporarily increase during scanning, which is normal. Resources are released immediately after the scan completes. For large disks or many files, scanning may take a few minutes, but you can continue using other applications during this time.

**Q: Can cleanup operations be undone?**
A: Cleanup operations such as deleting temporary files or emptying the Recycle Bin cannot be directly undone, as the files are permanently deleted. For system configuration changes such as power plan switching or hibernation settings, the program records change history and supports one-click rollback to the previous state.

**Q: What does the privacy scan do?**
A: Before packaging a release, `scripts/PrivacyScan.ps1` scans all text files for the following:

- GitHub tokens / API keys / private keys
- Connection strings containing passwords
- Local absolute paths (e.g., `C:\Users\YourUsername`)

This helps prevent accidental exposure of personal information during release.

**Q: How do I back up the current configuration?**
A: The program supports one-click backup and restore of all local data, including approval plans, execution history, operation logs, and settings. Backup files are stored in `%LocalAppData%\AetherOS\Backups\` and can be accessed via Settings or the Recovery Center.

**Q: What should I do first when using the program for the first time?**
A: We recommend the following order:

1. Launch the program with administrator privileges.
2. Go to "System Overview" and run a quick scan.
3. Review the scan results.
4. Go to "Storage Analysis" or "Clean & Optimize" to see reclaimable space.
5. Read the description and risk level for each item before confirming any action.
6. After execution, check the history in the "Recovery Center."

**Q: Does the program support custom rules?**
A: Yes. You can manage local rule packs in the "Rule Extensions" section. Rules are defined in declarative JSON format and do not support arbitrary DLL execution, ensuring safety and control.

------

## ⚠️ Disclaimer

This software is provided "as is" without any warranties. Use at your own risk. The author is not responsible for any data loss, system corruption, or other damages resulting from the use of this tool. All system changes are user-initiated and reversible where supported.

------

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

------

## 📜 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0). You are free to use, modify, and distribute this software, but any modified versions used as a network service must also be open-sourced under AGPL-3.0. For closed-source commercial use, a separate commercial license is required. Please see the [LICENSE](LICENSE) file for details.

------

## 🙏 Acknowledgments

- Built with ❤️ using .NET and WPF.
- Thanks to all contributors and users for their support.

------

## ☕ Support This Project

If this project has been helpful to you, feel free to buy the author a cup of milk tea ❤️

| WeChat | Alipay |
|:---:|:---:|
| ![WeChat](assets/wechat_pay.png) | ![Alipay](assets/alipay_pay.png) |
