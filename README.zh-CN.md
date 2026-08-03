#  AetherOS Guardian

**纯本地 Windows 系统检查、清理、配置与恢复工具**

[![Build Status](https://github.com/SKunAether/AetherOS/actions/workflows/build.yml/badge.svg)](https://github.com/SKunAether/AetherOS/actions/workflows/build.yml)
[![Release](https://img.shields.io/github/v/release/SKunAether/AetherOS)](https://github.com/SKunAether/AetherOS/releases)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Tech](https://img.shields.io/badge/Stack-Rust%20%7C%20Tauri%202%20%7C%20React%2019-blue)](Cargo.toml)

[**简体中文**](README.zh-CN.md) | [**English**](README.md)

---

> ✅ **v2.0 架构重构完成**：AetherOS 已从 .NET 8 / WPF 迁移到 **Rust + Tauri 2 + React 19**
> 技术栈，采用模块化 Capability-Provider 平台架构。实施进度见 [docs/ROADMAP.md](docs/ROADMAP.md)。

---

## 📋 简介

**AetherOS Guardian** 是一款模块化 Windows 系统优化与管理平台，提供全面的系统维护能力。支持 **基于本地规则的离线分析**（默认）和 **可选的云端 AI 分析**（通过多个 AI 厂商）。

与传统优化工具不同，AetherOS Guardian 强调 **可解释的规则**、**用户控制的执行流程** 以及 **完整的可审计性**。所有系统变更都需要用户明确确认，支持的操作会被记录并可回滚。

### 核心亮点
- **本地优先**：核心功能完全离线可用，AI 分析为可选功能，默认关闭。
- **用户掌控**：每一项系统修改都必须经过用户批准。
- **可审计、可恢复**：完整的操作历史，支持一键回滚。
- **可选 AI 分析**：支持多厂商切换（OpenAI 兼容、Anthropic Claude）—— 默认关闭，数据仅发送到您配置的厂商。
- **开源透明**：代码公开，可定制，社区驱动。

---

## ✨ 功能特性

| 功能 | 说明 |
| :--- | :--- |
| 🖥️ **系统概览** | 仪表板显示系统健康状况、磁盘使用率和快速状态。 |
| 🔍 **本地系统检查** | 基于规则的检查，结果直接链接到对应处理模块。 |
| 📦 **空间分析** | 识别并清理临时文件、缓存和大量无用数据。 |
| ⚡ **系统调优** | 管理电源计划、休眠、虚拟内存和启动项。 |
| 🔄 **恢复中心** | 完整的操作历史，支持一键回滚已支持的操作。 |
| 🧩 **规则扩展** | 通过本地规则包扩展功能（不支持任意 DLL 执行）。 |
| 🛠️ **系统托盘** | 最小化到托盘，快速访问，支持计划后台扫描。 |
| 🤖 **AI 分析**（可选） | 对扫描结果进行云端 AI 智能分析，支持多厂商切换（OpenAI、DeepSeek、Claude 等）。 |
| 🔐 **安全与隐私** | 内置隐私扫描器，防止意外泄露个人信息。AI 分析为可选。 |
| 📥 **自动更新** | 基于 GitHub Releases 的更新检查，支持 SHA-256 校验。 |

---

## 🚀 快速开始

### 普通用户（下载安装包）
1. 进入 [Releases](https://github.com/SKunAether/AetherOS/releases) 页面。
2. 下载最新的 `AetherOS-Guardian-{版本号}-win-x64.zip`。
3. 解压并运行 `AetherOS.Guardian.exe`。

> **注意**：如需使用全部功能（如修改电源计划、创建系统还原点等），请右键选择 **“以管理员身份运行”**。

### 开发者（从源码构建）

#### 环境要求
- Windows 10 / Windows 11
- [Rust 1.80+](https://www.rust-lang.org/)（含 MSVC 链接器）
- [Node.js 20+](https://nodejs.org/) 与 [pnpm](https://pnpm.io/)
- WebView2（Windows 11 自带，Windows 10 需安装）

#### 安装依赖
```bash
# 前端依赖（含 @tauri-apps/cli）
cd apps/desktop/frontend
pnpm install
```

#### 开发运行
```bash
cd apps/desktop/frontend
pnpm tauri dev
```

#### 构建安装包
```bash
cd apps/desktop/frontend
pnpm tauri build
# 产物：apps/desktop/src-tauri/target/release/bundle/nsis/*.exe
```

#### 生成本地发布包

powershell

```
.\scripts\Release.ps1 -GitHubOwner "SKunAether" -GitHubRepository "AetherOS"
```

生成的文件位于 `artifacts/release/` 目录。

------

## 📂 项目结构

```
AetherOS/
├── .cargo/                           # Rust 编译器配置
│   └── config.toml                   # 设置编译目标、链接器等参数
├── .github/                          # GitHub 自动化配置
│   ├── ISSUE_TEMPLATE/               # Issue 模板
│   │   ├── bug_report.md             # 报告 Bug 的模板
│   │   └── feature_request.md        # 请求新功能的模板
│   ├── workflows/                    # CI/CD 工作流
│   │   ├── build.yml                 # 每次推送/PR 时运行构建检查
│   │   ├── nightly.yml               # 每日构建（可选）
│   │   └── release.yml               # 打 tag 时触发发布（构建安装包、生成更新清单）
│   ├── dependabot.yml                # 依赖自动更新配置
│   ├── FUNDING.yml                   # 赞助信息
│   └── PULL_REQUEST_TEMPLATE.md      # PR 描述模板
├── apps/                             # 应用程序集合
│   └── desktop/                      # 桌面端应用（主程序）
│       ├── frontend/                 # React 前端（TypeScript + Tailwind CSS）
│       │   ├── dist/                 # 前端构建产物（Vite 打包输出）
│       │   │   ├── assets/           # 编译后的 JS/CSS 文件
│       │   │   │   ├── index-BuvkXy0c.js
│       │   │   │   ├── index-DVuYSGJf.css
│       │   │   │   └── window-CVrULaN6.js
│       │   │   └── index.html        # 入口 HTML
│       │   ├── public/               # 静态资源（图标、manifest 等）
│       │   ├── src/                  # 前端源代码
│       │   │   ├── api/              # 调用 Tauri 后端的 API 封装
│       │   │   │   ├── ai.ts         # AI 分析相关接口
│       │   │   │   ├── invoke.ts     # Tauri invoke 封装
│       │   │   │   ├── providers.ts  # 能力提供者（插件）API
│       │   │   │   ├── runtime.ts    # 运行时信息接口
│       │   │   │   ├── settings.ts   # 设置读写
│       │   │   │   └── system.ts     # 系统信息（硬件、OS 等）
│       │   │   ├── app/              # 应用全局布局和入口
│       │   │   │   ├── layouts/
│       │   │   │   │   └── AppLayout.tsx   # 主布局（侧边栏 + 内容区）
│       │   │   │   └── App.tsx       # 根组件（路由、主题）
│       │   │   ├── components/       # 可复用的 UI 组件
│       │   │   │   ├── charts/
│       │   │   │   │   └── Sparkline.tsx  # 迷你趋势图
│       │   │   │   └── ui/           # 基础组件（按钮、卡片等）
│       │   │   │       ├── button.tsx
│       │   │   │       ├── card.tsx
│       │   │   │       ├── placeholder.tsx
│       │   │   │       └── progress.tsx
│       │   │   ├── features/         # 各功能页面（按业务划分）
│       │   │   │   ├── cleaner/      # 清理与优化
│       │   │   │   │   └── CleanerPage.tsx
│       │   │   │   ├── dashboard/    # 系统概览仪表板
│       │   │   │   │   └── DashboardPage.tsx
│       │   │   │   ├── monitor/      # 系统监控（资源占用）
│       │   │   │   │   └── MonitorPage.tsx
│       │   │   │   ├── optimizer/    # 系统调优（启动项、电源等）
│       │   │   │   │   └── OptimizerPage.tsx
│       │   │   │   ├── recovery/     # 恢复中心（操作历史、回滚）
│       │   │   │   │   └── RecoveryPage.tsx
│       │   │   │   └── settings/     # 设置页面（含 AI 配置）
│       │   │   │       ├── AboutPage.tsx
│       │   │   │       └── SettingsPage.tsx
│       │   │   ├── hooks/            # 自定义 React Hooks
│       │   │   │   ├── use-backend-event.ts  # 监听后端事件
│       │   │   │   ├── use-interval.ts        # 定时轮询
│       │   │   │   ├── use-providers.ts       # 获取可用能力提供者
│       │   │   │   ├── use-settings.ts        # 读写设置
│       │   │   │   └── use-system-info.ts     # 获取系统信息
│       │   │   ├── i18n/             # 国际化配置
│       │   │   │   └── index.ts      # i18n 初始化
│       │   │   ├── locales/          # 语言文件（JSON）
│       │   │   │   ├── en-US/        # 英文
│       │   │   │   │   ├── common.json
│       │   │   │   │   ├── dashboard.json
│       │   │   │   │   └── nav.json
│       │   │   │   └── zh-CN/        # 简体中文
│       │   │   │       ├── common.json
│       │   │   │       ├── dashboard.json
│       │   │   │       └── nav.json
│       │   │   ├── stores/           # 状态管理（Zustand）
│       │   │   │   └── app-store.ts  # 全局应用状态
│       │   │   ├── styles/           # 全局样式（Tailwind 入口）
│       │   │   │   └── globals.css
│       │   │   ├── types/            # TypeScript 类型声明
│       │   │   │   └── index.ts      # 共享类型定义
│       │   │   ├── main.tsx          # 前端入口
│       │   │   └── vite-env.d.ts     # Vite 环境类型
│       │   ├── .prettierignore       # Prettier 忽略文件
│       │   ├── .prettierrc.json      # 代码格式化配置
│       │   ├── eslint.config.js      # ESLint 规则配置
│       │   ├── index.html            # HTML 模板
│       │   ├── package.json          # 前端依赖清单
│       │   ├── postcss.config.js     # PostCSS 配置（Tailwind）
│       │   ├── tailwind.config.js    # Tailwind CSS 配置
│       │   ├── tsconfig.json         # TypeScript 编译配置
│       │   ├── tsconfig.node.json    # Node 环境 TypeScript 配置
│       │   └── vite.config.ts        # Vite 构建配置
│       └── src-tauri/                # Tauri 后端（Rust）
│           ├── capabilities/         # 能力声明（权限控制）
│           │   └── default.json      # 默认允许的权限
│           ├── gen/                  # 自动生成的 schema（供前端调用）
│           │   └── schemas/
│           │       ├── acl-manifests.json
│           │       ├── capabilities.json
│           │       ├── desktop-schema.json
│           │       └── windows-schema.json
│           ├── icons/                # 应用图标
│           │   └── icon.ico          # Windows 图标
│           ├── src/                  # Rust 源代码
│           │   ├── ai/               # AI 分析模块
│           │   │   └── mod.rs        # AI 服务调用逻辑
│           │   ├── commands/         # Tauri 命令（暴露给前端的函数）
│           │   │   ├── ai.rs         # AI 分析命令
│           │   │   ├── cleaner.rs    # 清理操作命令
│           │   │   ├── hibernate.rs  # 休眠管理
│           │   │   ├── mod.rs        # 命令注册入口
│           │   │   ├── power.rs      # 电源计划
│           │   │   ├── quick_scan.rs # 快速扫描
│           │   │   ├── runtime.rs    # 运行时信息
│           │   │   ├── settings.rs   # 设置读写
│           │   │   └── system.rs     # 系统信息
│           │   ├── providers/        # 能力提供者（插件）示例
│           │   │   ├── demo.rs       # 演示提供者
│           │   │   └── mod.rs        # 提供者注册
│           │   ├── error.rs          # 统一错误类型
│           │   ├── lib.rs            # 库入口（供其他 crate 引用）
│           │   ├── main.rs           # 主函数（启动 Tauri）
│           │   ├── settings.rs       # 设置管理（读取 tauri.conf.json）
│           │   ├── startup.rs        # 启动项管理
│           │   ├── state.rs          # 应用全局状态（如配置）
│           │   └── tray.rs           # 系统托盘菜单及事件
│           ├── build.rs              # 构建脚本（编译前生成资源）
│           ├── Cargo.toml            # 后端依赖清单
│           └── tauri.conf.json       # Tauri 核心配置（应用名、窗口、权限等）
├── assets/                           # 静态资源（图片、图标、捐赠码）
│   ├── icons/
│   │   ├── AetherOS.ico              # 应用图标（多尺寸）
│   │   └── AetherOS.png              # 图标 PNG 版本
│   ├── alipay_pay.png                # 支付宝收款码
│   └── wechat_pay.png                # 微信收款码
├── crates/                           # Rust 工作空间（模块化能力实现）
│   ├── aether-cleaner/               # 清理能力（删除临时文件、缓存等）
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── provider.rs           # 实现 CapabilityProvider trait
│   │   └── Cargo.toml
│   ├── aether-core/                  # 核心数据模型与 trait 定义（所有 crate 共享）
│   │   ├── src/
│   │   │   ├── errors/               # 错误类型（与能力相关）
│   │   │   │   ├── capability.rs
│   │   │   │   └── mod.rs
│   │   │   ├── events/               # 事件总线相关类型
│   │   │   │   └── mod.rs
│   │   │   ├── models/               # 业务模型（扫描结果、操作记录、设置等）
│   │   │   │   ├── action.rs         # 操作定义（可执行动作）
│   │   │   │   ├── ai.rs             # AI 分析结果
│   │   │   │   ├── cleaner.rs        # 可清理项
│   │   │   │   ├── deep_scan.rs      # 深度扫描结果
│   │   │   │   ├── diagnostics.rs    # 诊断信息
│   │   │   │   ├── hibernate.rs      # 休眠状态
│   │   │   │   ├── maintenance.rs    # 维护任务
│   │   │   │   ├── mod.rs
│   │   │   │   ├── operation.rs      # 操作历史记录
│   │   │   │   ├── pagefile.rs       # 虚拟内存配置
│   │   │   │   ├── plugin.rs         # 插件描述
│   │   │   │   ├── power.rs          # 电源计划
│   │   │   │   ├── quick_scan.rs     # 快速扫描结果
│   │   │   │   ├── restore_point.rs  # 系统还原点
│   │   │   │   ├── settings.rs       # 应用设置
│   │   │   │   └── system_context.rs # 系统上下文（快照）
│   │   │   ├── traits/               # 能力提供者接口
│   │   │   │   ├── capability_provider.rs
│   │   │   │   └── mod.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── aether-monitor/               # 系统监控能力（CPU/内存/磁盘）
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── provider.rs
│   │   └── Cargo.toml
│   ├── aether-optimizer/             # 系统优化能力（启动项、服务调整）
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── provider.rs
│   │   │   └── startup.rs            # 启动项管理实现
│   │   └── Cargo.toml
│   ├── aether-plugin-sdk/            # 插件开发 SDK（供第三方插件引用）
│   │   ├── src/
│   │   │   └── lib.rs                # 导出插件所需宏和类型
│   │   └── Cargo.toml
│   ├── aether-recovery/              # 恢复能力（操作历史、回滚）
│   │   ├── src/
│   │   │   ├── history.rs            # 历史记录存储与查询
│   │   │   ├── lib.rs
│   │   │   └── provider.rs
│   │   └── Cargo.toml
│   ├── aether-runtime/               # 运行时核心（插件管理、事件总线、调度）
│   │   ├── src/
│   │   │   ├── bus/                  # 事件总线（模块间通信）
│   │   │   │   └── mod.rs
│   │   │   ├── plugin/               # 插件加载、生命周期管理
│   │   │   │   └── mod.rs
│   │   │   ├── registry/             # 能力注册中心
│   │   │   │   └── mod.rs
│   │   │   ├── scheduler/            # 后台任务调度（定时扫描等）
│   │   │   │   └── mod.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── aether-system/                # 系统信息获取（硬件、OS、注册表、服务）
│       ├── src/
│       │   ├── filesystem/           # 文件系统操作
│       │   │   └── mod.rs
│       │   ├── hardware/             # 硬件信息（CPU、内存等）
│       │   │   └── mod.rs
│       │   ├── power/                # 电源管理（API 调用）
│       │   │   └── mod.rs
│       │   ├── process/              # 进程管理
│       │   │   └── mod.rs
│       │   ├── registry/             # Windows 注册表读写
│       │   │   └── mod.rs
│       │   ├── service/              # Windows 服务管理
│       │   │   └── mod.rs
│       │   └── lib.rs
│       └── Cargo.toml
├── docs/                             # 扩展文档
│   ├── ARCHITECTURE.md               # 架构设计（英文）
│   ├── ARCHITECTURE.zh-CN.md         # 架构设计（中文）
│   ├── plugin-development.md         # 插件开发指南（英文）
│   ├── plugin-development.zh-CN.md   # 插件开发指南（中文）
│   ├── PROJECT-STRUCTURE.md          # 项目结构说明（英文）
│   ├── RELEASE-GUIDE.md              # 发布流程指南
│   ├── ROADMAP.md                    # 开发路线图（英文）
│   └── ROADMAP.zh-CN.md              # 开发路线图（中文）
├── installer/                        # 安装器相关文件
│   ├── scripts/                      # 安装脚本（待用）
│   ├── windows/                      # Windows 专用安装配置
│   │   └── README.md                 # 安装说明
│   └── update-manifest.template.json # 更新清单模板（用于自动更新）
├── plugins/                          # 插件定义（声明式配置）
│   ├── builtin/                      # 内置插件（随主程序发布）
│   │   ├── developer-environment/    # 开发者环境检测
│   │   │   └── plugin.json
│   │   ├── storage-analysis/         # 存储分析
│   │   │   └── plugin.json
│   │   └── windows-cleaner/          # Windows 标准清理
│   │       └── plugin.json
│   └── examples/                     # 示例插件（供开发者参考）
│       └── sample-rule-pack/
│           ├── plugin.json           # 插件元数据（名称、版本、能力声明）
│           └── README.md             # 示例说明
├── .editorconfig                     # 编辑器统一配置
├── .env.example                      # 环境变量示例（CI 等）
├── .gitignore                        # Git 忽略规则（含 target、node_modules）
├── Cargo.lock                        # Rust 依赖锁定文件
├── Cargo.toml                       # Rust 工作空间根清单（定义所有子 crate）
├── CHANGELOG.md                      # 更新日志（英文）
├── CHANGELOG.zh-CN.md                # 更新日志（中文）
├── CODE_OF_CONDUCT.md                # 行为守则
├── CONTRIBUTING.md                   # 贡献指南
├── LICENSE                           # AGPL-3.0 许可证
├── package.json                      # pnpm 工作空间根（用于前端统一管理）
├── pnpm-lock.yaml                    # pnpm 锁定文件
├── pnpm-workspace.yaml               # pnpm 工作空间配置（指向 apps/desktop/frontend）
├── README.md                         # 项目说明（英文）
├── README.zh-CN.md                   # 项目说明（中文）
├── rust-toolchain.toml               # 指定 Rust 工具链版本
└── SECURITY.md                       # 安全策略（漏洞报告流程）
```



------

## ❓ 常见问题

**问：程序需要联网吗？**
答：核心功能（系统检查、清理、配置）完全离线运行，不收集任何遥测信息。如果你在设置中启用了"AI 自动研判"，系统扫描数据会发送到你配置的 AI 服务商。AI 分析**默认关闭**，需要用户明确启用。

**问：程序有 AI 功能吗？**
答：AetherOS Guardian 提供可选的云端 AI 分析功能，可以对扫描结果进行智能研判。支持多厂商切换（OpenAI、DeepSeek、Claude、千问等任意 OpenAI 兼容接口）。该功能**默认关闭**——你需要在设置中明确启用并配置自己的 API 密钥。API 密钥使用 Windows DPAPI 加密存储在本机。启用后，系统扫描数据（磁盘使用率、文件大小等，不含个人文件内容）会发送到你配置的 AI 服务商。

**问：为什么需要管理员权限？**
答：部分功能需要提升权限才能正常工作，例如：

- 切换电源计划
- 启用/禁用休眠
- 创建系统还原点
- 访问受系统保护的目录
- 修改虚拟内存设置

如果你只使用系统概览和空间分析等只读功能，可以不以管理员身份运行，但功能会受限。

**问：程序会持续在后台运行吗？**
答：默认情况下不会。程序只在用户主动打开时运行。如果你启用了“关闭到托盘”功能，程序会常驻系统托盘；如果你启用了“计划扫描”，程序会按设定时间在后台执行扫描任务。除此之外，不会进行任何未经用户触发的后台活动。

**问：扫描和清理会占用多少系统资源？**
答：扫描过程中 CPU 和磁盘占用率会略有升高，这是正常的。扫描结束后资源会立即释放。对于大型磁盘或大量文件，扫描可能需要几分钟，但在此期间你可以正常使用其他应用程序。

**问：清理操作能撤销吗？**
答：清理操作（如删除临时文件、清空回收站）本身不可直接撤销，因为文件已被永久删除。对于系统配置类操作（如电源计划切换、休眠启用/禁用），程序会记录变更历史，并支持一键回滚到修改前的状态。

**问：隐私扫描是做什么的？**
答：在打包发布前，`scripts/PrivacyScan.ps1` 脚本会扫描所有文本文件，查找以下内容：

- GitHub Token / API Key / 私钥
- 带密码的连接字符串
- 本机绝对路径（如 `C:\Users\你的用户名`）

防止在发布时意外泄露个人信息。

**问：如何备份当前配置？**
答：程序支持一键备份和恢复所有本地数据，包括审批计划、执行历史、操作日志和设置。备份文件存储在 `%LocalAppData%\AetherOS\Backups\` 目录下，可在设置或恢复中心中找到入口。

**问：第一次使用时应该先做什么？**
答：建议按以下顺序操作：

1. 以管理员身份启动程序
2. 进入“系统概览”，点击快速扫描
3. 查看扫描结果
4. 进入“空间分析”或“清理与优化”查看可清理的空间
5. 确认操作前查看每个项目的说明和风险等级
6. 执行操作后，可在“恢复中心”查看历史记录

**问：程序支持自定义规则吗？**
答：支持。你可以在“规则扩展”页面管理本地规则包。规则以声明式 JSON 格式定义，不支持任意 DLL 执行，确保安全可控。

------

## ⚠️ 免责声明

本软件按“现状”提供，不提供任何明示或暗示的担保。使用本软件的风险由用户自行承担。作者不对因使用本工具造成的任何数据丢失、系统损坏或其他损失负责。所有系统变更均由用户发起，并在支持的情况下可逆。

------

## 📝 更新日志

版本历史请查阅 [CHANGELOG.md](CHANGELOG.zh-CN.md)。

------

## 📜 许可证

本项目采用 GNU Affero 通用公共许可证 v3.0 (AGPL-3.0) 授权。你可以自由使用、修改和分发本软件，但任何作为网络服务提供的修改版本，也必须以 AGPL-3.0 许可开源。如需闭源商用，则需另行获取单独的商用许可。详情请见 [LICENSE](LICENSE) 文件。
------

## 🙏 致谢

- 使用 ❤️ 和 Rust / Tauri / React 构建。
- 感谢所有贡献者和用户的支持。

------

## ☕ 支持项目

如果这个项目对你有帮助，欢迎请作者喝杯奶茶 ❤️

| 微信 | 支付宝 |
|:---:|:---:|
| ![微信](assets/wechat_pay.png) | ![支付宝](assets/alipay_pay.png) |
