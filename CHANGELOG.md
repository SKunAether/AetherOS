# Changelog

All notable changes to AetherOS Guardian will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.0.0] - 2026-08-03

### Architecture Rewrite (Complete)
- **Full feature set**: Cleaner (4 rules, full chain), real-time monitoring, power plans / hibernate / startup items / virtual memory, action history / restore points.
- **System tray**: tray icon, close-to-tray, run-at-startup, silent startup (`--tray` arg).
- **AI analysis**: multi-provider (OpenAI-compatible / Anthropic), DPAPI-encrypted API keys, one-click system analysis.
- **Theme system**: complete CSS-variable dual theme (dark/light), native window title bar follows theme.
- **About page**: CCSwitch-style redesign, removed hostname privacy leak, added website / GitHub / changelog / check-update.
- **Engineering**: CI (fmt + clippy + build), bilingual docs, per-file project structure documentation.

---

## [2.3.0] - 2026-07-23

### Added
- **AI Comprehensive Analysis**: Expanded system data collection (power plan, hibernate, virtual memory, system restore, diagnostics, execution history) for smarter AI recommendations.
- **AI Auto-Execution**: Approved AI recommendations now execute automatically via system tools (hibernate toggle via powercfg, power plan switching) instead of just navigating.
- **AI User Consent Flow**: AI analysis no longer triggers automatically. User clicks "AI Deep Analysis" button, sees confirmation dialog, then analysis runs.
- **Recovery Center Rollback**: Each rollback-capable operation now shows a ↩ rollback button. Hibernate and power plan changes can be rolled back with one click.
- **Provider Quick-Add Presets**: One-click add for DeepSeek, OpenAI, Qwen, Groq, Google Gemini, and Anthropic Claude.
- **AI Action Approval Flow**: Recommendations shown with checkboxes, select/deselect all, and "Execute Selected" with execution feedback.
- **NuGet Supply Chain Audit**: Enabled `NuGetAudit` across all projects.

### Changed
- **SystemContextBuilder**: Rewritten with 7 data dimensions for comprehensive AI analysis.
- **AI Card Position**: Moved from page bottom to prominent position below scan header.
- **PrivacyScan**: Added Claude API key pattern (`sk-ant-*`) detection.

### Fixed
- AI no longer fires on page load before scan data exists.
- AI card now shows entry hint when AI is disabled (not completely hidden).
- .gitignore deduplication and cleanup.

### Security
- Enhanced PrivacyScan regex patterns for broader secret detection.
- Providers.json and `.claude/` directory added to .gitignore.

## [2.2.0] - 2026-07-22

### Added
- **System Overview Dashboard**: Real-time display of system health, disk usage, and quick scan status.
- **Local System Inspection**: Rule-based system inspection with results linked directly to handling modules.
- **Storage Analysis**: Identify and clean temporary files, caches, and large unused data.
- **System Tuning**: Manage power plans, hibernation, virtual memory, and startup items.
- **Recovery Center**: Full operation history with one-click rollback for supported actions.
- **Rule Extensions**: Extend functionality via local declarative rule packs (no arbitrary DLL execution).
- **System Tray**: Minimize to tray, quick access, and scheduled background scans.
- **Privacy Scanner**: Built-in privacy scanning to prevent accidental leakage of personal data before release.
- **Auto-Update**: GitHub Releases based update checker with SHA-256 verification and independent updater.
- **Modular Build Scripts**: Clean, build, test, package, and release scripts under `scripts/`.
- **GitHub Actions**: CI/CD automation for build and tag-based release workflows.
- **Application Icons**: Complete icon set for window, taskbar, tray, and installer.
- **AI Automatic Analysis** (opt-in): Multi-provider cloud AI support (OpenAI-compatible, Anthropic Claude).
- **AI Provider Management**: CC-Switch style UI for configuring and switching AI providers.
- **AI Analysis Cards**: Results display on Dashboard, System Check, System Configuration, and Recovery pages.
- **DPAPI Key Encryption**: API keys encrypted and stored locally via Windows Data Protection API.
- **Graceful AI Fallback**: On AI failure, existing rule-based analysis remains unchanged.

### Changed
- Replaced the monolithic release script with modular PowerShell scripts.
- Unified versioning across all projects via `Directory.Build.props`.

### Fixed
- Dashboard disk capacity loading issue.
- Update module compilation errors.
- XAML parsing and binding issues in Settings and System Configuration pages.
- PowerShell script encoding issues (UTF-8 with BOM).

### Security
- Added `scripts/PrivacyScan.ps1` to scan for tokens, private keys, and local absolute paths before packaging.
- Added `.gitignore` rules for certificates, private keys, logs, and build artifacts.

---

## [2.1.0] - 2026-07-21

### Added
- GitHub-compatible update manifest and independent updater.
- Unified application icon resources across all UI surfaces.
- Refined dashboard and inspection navigation.

### Changed
- Improved system configuration module layout and usability.
- Enhanced power plan and hibernation services.

### Fixed
- Power plan display and switching issues on non-English Windows systems.
- System restore point creation error handling.

---

## [2.0.0] - 2026-07-20

### Changed
- **Major rework**: Removed all AI providers, models, API keys, and AI assistant functionality.
- Repositioned AetherOS as a pure-local Windows maintenance tool.
- Rebuilt the user interface to focus on system inspection, cleanup, and recovery.

### Removed
- Eira AI assistant and all related services.
- AI Provider configuration pages.
- All OpenAI, Ollama, and Hybrid provider integrations.

---

[2.3.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.3.0
[2.2.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.2.0
[2.1.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.1.0
[2.0.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.0.0