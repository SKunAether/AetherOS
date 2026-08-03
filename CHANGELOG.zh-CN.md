# 更新日志

本文档记录 AetherOS Guardian 的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [2.0.0] - 2026-08-03

### 架构重写（完成）
- **完整功能落地**：清理（4 规则全链路）、实时监控、电源计划/休眠/启动项/虚拟内存、操作历史/还原点。
- **系统托盘**：托盘图标、关闭到托盘、开机自启、静默启动（`--tray` 参数）。
- **AI 分析**：多厂商（OpenAI 兼容 / Anthropic），API Key DPAPI 加密，一键系统分析。
- **主题系统**：完整 CSS 变量双主题（深/浅），原生窗口标题栏跟随主题。
- **关于页**：CCSwitch 风格重做，移除主机名隐私泄露，含官网/GitHub/更新日志/检查更新。
- **工程化**：CI（fmt + clippy + 构建）、双语文档与项目结构文档（逐文件注释）。

### 架构重写（进行中）
- **技术栈迁移**：从 .NET 8 / WPF 迁移到 **Rust + Tauri 2 + React 19 + TypeScript + TailwindCSS**。
- **模块化 Capability-Provider 平台架构**：新增 `crates/aether-core`（领域模型/trait/错误/事件）、
  `aether-runtime`（注册中心/事件总线/调度器）、`aether-system`（Windows API 封装）。
- **前端重构**：Feature-Sliced 组织、深色玻璃拟态主题、中英双语（react-i18next）、
  Zustand + TanStack Query 状态管理、React Router 7 路由。
- **工程化**：Cargo Workspace + pnpm monorepo、CI 工作流（build/release/nightly）、
  ESLint/Prettier/Clippy/rustfmt 代码规范。
- 完整重构方案见《AetherOS 2.0 架构重构方案 v1.1（完善版）.md》，进度见 docs/ROADMAP.md。
- v1（C#/WPF）源码保留于 `src/` 作为历史参考。

---

## [2.3.0] - 2026-07-23

### 新增
- **AI 综合研判**：扩展系统数据采集覆盖 7 个维度（电源计划、休眠、虚拟内存、系统还原、诊断、执行历史），AI 可进行全面分析。
- **AI 自动执行**：已审批的 AI 建议现可通过系统工具自动执行（休眠切换 via powercfg、电源计划切换），无需手动跳转。
- **AI 用户授权流程**：AI 分析不再自动触发。用户点击「AI 深度分析」按钮→弹出确认对话框→确认后运行分析。
- **恢复中心回滚**：每条可回滚的操作记录新增 ↩ 回滚按钮，休眠和电源计划变更可一键恢复。
- **厂商快捷预设**：一键添加 DeepSeek、OpenAI、千问、Groq、Google Gemini、Anthropic Claude。
- **AI 审批执行流**：建议以勾选框展示，支持全选/取消全选、执行所选并显示执行反馈。

### 变更
- **SystemContextBuilder 重写**：7 维度数据收集，AI 分析更全面准确。
- **AI 卡片位置**：从页面底部移到显眼位置（扫描标题下方）。
- **隐私扫描增强**：新增 Claude API 密钥模式（`sk-ant-*`）检测。

### 修复
- AI 不再在页面加载时自动触发（无扫描数据时）。
- AI 禁用时显示入口提示条，而非完全隐藏。
- .gitignore 去重和清理。

### 安全
- 增强 PrivacyScan 正则模式，覆盖更多密钥类型。
- providers.json 和 `.claude/` 目录加入 .gitignore。

## [2.2.0] - 2026-07-22

### 新增
- **AI 自动研判**（可选）：多厂商云端 AI 支持（OpenAI 兼容、Anthropic Claude），默认关闭。
- **AI 厂商管理**：CC-Switch 风格的左右分栏 UI，自由添加、配置和切换 AI 厂商。
- **AI 分析卡片**：在仪表盘、系统检查、系统配置、恢复中心等多个页面展示 AI 分析结果。
- **DPAPI 密钥加密**：API 密钥通过 Windows Data Protection API 加密存储在本机。
- **AI 优雅降级**：AI 分析失败时自动回退到已有规则分析，不影响核心功能。
- **系统概览仪表板**：实时显示系统健康状况、磁盘使用率和快速扫描状态。
- **本地系统检查**：基于规则的检查，结果直接链接到对应处理模块。
- **空间分析**：识别并清理临时文件、缓存和大量无用数据。
- **系统调优**：管理电源计划、休眠、虚拟内存和启动项。
- **恢复中心**：完整的操作历史，支持一键回滚已支持的操作。
- **规则扩展**：通过本地声明式规则包扩展功能（不支持任意 DLL 执行）。
- **系统托盘**：最小化到托盘，快速访问，支持计划后台扫描。
- **隐私扫描器**：发布前自动扫描，防止意外泄露令牌、私钥和本机路径。
- **自动更新**：基于 GitHub Releases 的更新检查，支持 SHA-256 校验和独立更新器。
- **模块化构建脚本**：`scripts/` 下的清理、构建、测试、打包和发布脚本。
- **GitHub Actions**：CI/CD 自动化构建和基于标签的发布工作流。
- **应用图标**：完整的图标集，覆盖窗口、任务栏、托盘和安装器。

### 变更
- 将单体发布脚本拆分为模块化的 PowerShell 脚本。
- 通过 `Directory.Build.props` 统一所有项目的版本号。
- 更新文档和 README 以反映 AI 可选功能。

### 修复
- 仪表板磁盘容量加载问题。
- 更新模块编译错误。
- 设置页和系统配置页的 XAML 解析和绑定问题。
- PowerShell 脚本编码问题（统一为 UTF-8 with BOM）。

### 安全
- 增加 `scripts/PrivacyScan.ps1`，打包前扫描令牌、私钥和本机绝对路径。
- 增加 `.gitignore` 规则，排除证书、私钥、日志和构建产物。
- 启用 NuGet 供应链安全审计（`NuGetAudit`）。
- 增强 PrivacyScan 以检测更多类型的 API 密钥（Claude 等）。

---

## [2.1.0] - 2026-07-21

### 新增
- 基于 GitHub Releases 的更新清单和独立更新器。
- 统一所有界面中的应用图标资源。
- 优化仪表板和检查导航。

### 变更
- 改进系统配置模块的布局和可用性。
- 增强电源计划和休眠服务。

### 修复
- 非英文 Windows 系统上电源计划显示和切换问题。
- 系统还原点创建的错误处理。

---

## [2.0.0] - 2026-07-20

### 变更
- **重大重构**：移除所有 AI Provider、模型、API Key 及 AI 助手功能。
- 将 AetherOS 重新定位为纯本地 Windows 维护工具。
- 重建用户界面，聚焦于系统检查、清理和恢复。

### 移除
- Eira AI 助手及所有相关服务。
- AI Provider 配置页面。
- 所有 OpenAI、Ollama 和 Hybrid Provider 集成。

---

[2.3.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.3.0
[2.2.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.2.0
[2.1.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.1.0
[2.0.0]: https://github.com/SKunAether/AetherOS/releases/tag/v2.0.0