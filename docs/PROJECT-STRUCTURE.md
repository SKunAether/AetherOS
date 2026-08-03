# AetherOS 项目结构

> 本文档与仓库实际结构保持同步。每个文件后附用途注释。

## 概览

AetherOS 2.0 采用 **Cargo Workspace（Rust 后端）+ pnpm monorepo（React 前端）** 的组织方式。
桌面应用（`apps/desktop`）由 Tauri 2 壳层与 React 前端组成，核心能力以 8 个独立 crate 提供。

```
AetherOS/
├── Cargo.toml                      # Cargo Workspace 根配置：声明全部 crate 成员与统一依赖版本
├── Cargo.lock                      # Rust 依赖版本锁定文件（应用项目提交）
├── rust-toolchain.toml             # Rust 工具链版本锁定（1.97.1 + rustfmt/clippy 组件）
├── package.json                    # pnpm 根配置：全局脚本（dev/build/tauri/lint）
├── pnpm-workspace.yaml             # pnpm 工作空间：声明前端包与构建批准
├── pnpm-lock.yaml                  # pnpm 依赖锁文件
├── .gitignore                      # Git 忽略规则（target/node_modules/dist/密钥等）
├── .editorconfig                   # 编辑器格式统一（缩进/换行）
├── .env.example                    # 环境变量示例（更新源/日志级别/数据目录）
├── .cargo/config.toml              # Cargo 全局配置（增量编译）
├── README.md / README.zh-CN.md     # 项目介绍（英文/中文）
├── CHANGELOG.md / CHANGELOG.zh-CN.md  # 更新日志（英文/中文）
├── LICENSE                         # AGPL-3.0 开源协议
├── SECURITY.md                     # 安全披露政策
├── CODE_OF_CONDUCT.md              # 贡献者行为准则
├── CONTRIBUTING.md                 # 贡献指南
│
├── apps/desktop/                   # 桌面端应用
│   ├── frontend/                   # React 19 前端工程
│   │   ├── index.html              # HTML 入口
│   │   ├── package.json            # 前端依赖与脚本（含 @tauri-apps/cli）
│   │   ├── vite.config.ts          # Vite 配置（端口 1420、@ 路径别名、Tauri 适配）
│   │   ├── tsconfig.json           # TypeScript 编译配置
│   │   ├── tsconfig.node.json      # Node 侧工具配置（vite.config.ts）
│   │   ├── tailwind.config.js      # Tailwind 主题（CSS 变量双主题映射）
│   │   ├── postcss.config.js       # PostCSS 配置（Tailwind + Autoprefixer）
│   │   ├── eslint.config.js        # ESLint 扁平配置（TS/React Hooks/Prettier）
│   │   ├── .prettierrc.json        # Prettier 代码格式规范
│   │   ├── .prettierignore         # Prettier 忽略规则
│   │   └── src/
│   │       ├── main.tsx            # React 入口：挂载根组件 + QueryClient + i18n
│   │       ├── vite-env.d.ts       # Vite 客户端类型声明
│   │       ├── app/
│   │       │   ├── App.tsx         # 应用根组件：路由定义 + 主题/语言副作用
│   │       │   └── layouts/AppLayout.tsx  # 主布局：侧边栏导航 + Provider 计数 + 内容区
│   │       ├── components/
│   │       │   ├── ui/             # 基础 UI 组件（button/card/progress/placeholder）
│   │       │   └── charts/Sparkline.tsx  # SVG 迷你折线图（监控实时数据）
│   │       ├── features/           # 业务功能模块（Feature-Sliced）
│   │       │   ├── dashboard/DashboardPage.tsx  # 系统概览：健康分/磁盘/快捷入口
│   │       │   ├── cleaner/CleanerPage.tsx      # 清理：扫描/勾选/执行/结果
│   │       │   ├── optimizer/OptimizerPage.tsx  # 优化：电源计划/休眠/启动项/虚拟内存
│   │       │   ├── monitor/MonitorPage.tsx      # 监控：CPU/内存/磁盘实时图表
│   │       │   ├── recovery/RecoveryPage.tsx    # 恢复：操作历史/系统还原点
│   │       │   └── settings/
│   │       │       ├── SettingsPage.tsx         # 设置：外观/启动行为/AI 服务商
│   │       │       └── AboutPage.tsx            # 关于：产品信息/外链/技术栈
│   │       ├── stores/app-store.ts # Zustand 全局状态（主题/语言）+ 窗口主题同步
│   │       ├── hooks/              # 自定义 Hooks（系统信息/Provider/设置/事件/轮询）
│   │       ├── api/                # Tauri IPC 封装
│   │       │   ├── invoke.ts       #   统一 invoke + 事件订阅
│   │       │   ├── system.ts       #   系统信息 + 打开外链
│   │       │   ├── runtime.ts      #   Provider 列表/执行/扫描
│   │       │   ├── settings.ts     #   设置读写
│   │       │   ├── providers.ts    #   业务能力封装（清理/优化/监控/恢复）
│   │       │   └── ai.ts           #   AI 服务商与分析
│   │       ├── types/index.ts      # TS 类型定义（与 Rust serde camelCase 对齐）
│   │       ├── styles/globals.css  # 全局样式与 CSS 变量双主题系统
│   │       ├── i18n/index.ts       # react-i18next 初始化（中英双语）
│   │       ├── locales/            # 语言资源（zh-CN / en-US）
│   │       └── lib/utils.ts        # 工具函数（cn 类名合并）
│   │
│   └── src-tauri/                  # Tauri 2 壳层（Rust）
│       ├── Cargo.toml              # 桌面壳依赖（tauri + 各 aether crate）
│       ├── build.rs                # tauri-build 构建脚本
│       ├── tauri.conf.json         # Tauri 配置：窗口/主题/打包/前置命令
│       ├── capabilities/default.json  # 前端权限能力声明
│       ├── icons/icon.ico          # 应用图标
│       ├── gen/schemas/            # 构建时生成的权限 schema（勿手改）
│       └── src/
│           ├── main.rs             # 进程入口（调用 lib::run）
│           ├── lib.rs              # 应用装配：运行时/Provider/托盘/事件 relay/命令注册
│           ├── state.rs            # 全局共享状态 AppState（运行时/系统引擎/设置）
│           ├── error.rs            # 命令层统一错误（code + message）
│           ├── settings.rs         # 设置持久化（%LOCALAPPDATA%\AetherOS\Settings）
│           ├── startup.rs          # 开机自启（注册表 Run 键）
│           ├── tray.rs             # 系统托盘图标与菜单
│           ├── ai/mod.rs           # AI 服务：DPAPI 加密/服务商 CRUD/分析调用
│           ├── providers/          # 内置能力 Provider 组装
│           │   ├── mod.rs          #   注册 demo/cleaner/monitor/optimizer/recovery
│           │   └── demo.rs         #   演示 Provider（验证全链路）
│           └── commands/           # IPC 命令定义
│               ├── mod.rs          #   命令模块聚合
│               ├── system.rs       #   系统信息 / 打开外链
│               ├── runtime.rs      #   Provider 列表/执行/扫描 / ping / 事件测试
│               ├── settings.rs     #   设置读写 + 开机自启同步
│               ├── ai.rs           #   AI 服务商 CRUD / 连接测试 / 系统分析
│               ├── quick_scan.rs   #   快速扫描（占位）
│               ├── cleaner.rs      #   清理命令（占位，实际走 Provider）
│               ├── power.rs        #   电源计划命令（占位）
│               └── hibernate.rs    #   休眠命令（占位）
│
├── crates/                         # Rust 核心库（Cargo Workspace）
│   ├── aether-core/                # 核心领域层（无平台依赖）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs              #   模块汇总
│   │       ├── traits/capability_provider.rs  # CapabilityProvider trait（架构核心抽象）
│   │       ├── models/             #   领域模型（serde camelCase，与前端类型对齐）
│   │       │   ├── cleaner.rs      #     清理项/扫描结果/执行结果
│   │       │   ├── quick_scan.rs   #     快速扫描结果
│   │       │   ├── deep_scan.rs    #     深度扫描（文件类型统计/大文件）
│   │       │   ├── maintenance.rs  #     维护计划/动作/执行报告
│   │       │   ├── power.rs        #     电源计划
│   │       │   ├── hibernate.rs    #     休眠状态
│   │       │   ├── pagefile.rs     #     虚拟内存/分页文件
│   │       │   ├── restore_point.rs#     系统还原点
│   │       │   ├── diagnostics.rs  #     诊断快照/健康分
│   │       │   ├── system_context.rs  #   系统上下文快照
│   │       │   ├── ai.rs           #     AI 服务商/分析结果
│   │       │   ├── plugin.rs       #     插件清单
│   │       │   ├── settings.rs     #     应用设置
│   │       │   ├── action.rs       #     操作审计记录
│   │       │   └── operation.rs    #     操作/配置变更记录
│   │       ├── errors/capability.rs #   统一错误类型
│   │       └── events/mod.rs       #   全局事件（进度/状态/通知）
│   │
│   ├── aether-runtime/             # 能力运行时（调度中枢）
│   │   └── src/
│   │       ├── lib.rs              #   RuntimeContext 组装（registry+bus+scheduler）
│   │       ├── registry/mod.rs     #   Provider 注册中心（描述符查询）
│   │       ├── bus/mod.rs          #   事件总线（tokio broadcast）
│   │       ├── scheduler/mod.rs    #   任务调度器（后台任务/取消/事件上报）
│   │       └── plugin/mod.rs       #   插件加载器（Phase 6 扩展）
│   │
│   ├── aether-system/              # 系统引擎层（Windows API 封装）
│   │   └── src/
│   │       ├── lib.rs              #   SystemEngine 门面
│   │       ├── hardware/mod.rs     #   系统信息采集（windows-rs：CPU/内存/磁盘/OS/管理员）
│   │       ├── registry/mod.rs     #   注册表读写/枚举/删除
│   │       ├── process/mod.rs      #   命令执行/提权运行
│   │       ├── power/mod.rs        #   powercfg 封装（电源计划/休眠）
│   │       ├── filesystem/mod.rs   #   文件扫描/删除/重解析点检测
│   │       └── service/mod.rs      #   系统服务（预留）
│   │
│   ├── aether-cleaner/             # 清理能力 Provider（4 条规则：临时/更新缓存/缩略图）
│   ├── aether-optimizer/           # 优化能力 Provider（电源/休眠/启动项/虚拟内存）
│   ├── aether-monitor/             # 监控能力 Provider（sysinfo 实时采样）
│   ├── aether-recovery/            # 恢复能力 Provider（操作历史/系统还原点）
│   └── aether-plugin-sdk/          # 插件开发 SDK（prelude 导出核心类型）
│
├── plugins/                        # 插件目录
│   ├── builtin/                    # 内置插件清单（windows-cleaner/storage-analysis/developer-environment）
│   └── examples/sample-rule-pack/  # 示例规则包
│
├── installer/                      # 安装与发布配置
│   ├── update-manifest.template.json  # 自动更新清单模板
│   └── windows/README.md              # Windows 安装说明
│
├── docs/                           # 文档
│   ├── ARCHITECTURE.md / .zh-CN.md     # 架构说明（中英）
│   ├── PROJECT-STRUCTURE.md           # 本文档
│   ├── ROADMAP.md / ROADMAP.zh-CN.md   # 路线图（中英）
│   ├── RELEASE-GUIDE.md               # 发布指南
│   └── plugin-development.md / .zh-CN.md  # 插件开发文档（中英）
│
└── .github/                        # GitHub 配置
    ├── workflows/                  # CI/CD（build/release/nightly）
    ├── ISSUE_TEMPLATE/             # Issue 模板（bug/feature）
    ├── PULL_REQUEST_TEMPLATE.md    # PR 模板
    ├── dependabot.yml              # 依赖自动更新
    └── FUNDING.yml                 # 赞助配置
```

## 数据流

```
React 组件 → api/*（invoke 封装）→ Tauri 命令 → 能力运行时（ProviderRegistry）
     → Provider（execute/scan）→ aether-system（Windows API）
长任务 → 事件总线 → Tauri 事件通道 → React 监听器
```

## 关键说明

- **无测试代码**：按项目决策，测试代码已移除；质量保障依靠 CI（fmt + clippy + 构建）。
- **隐私**：前端不展示主机名/用户名等隐私信息；AI API Key 以 DPAPI 加密存储于
  `%LOCALAPPDATA%\AetherOS\Settings\providers.json`（已被 .gitignore 忽略）。
- **运行**：`cd apps/desktop/src-tauri && ../frontend/node_modules/.bin/tauri dev`
