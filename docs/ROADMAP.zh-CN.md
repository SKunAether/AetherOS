# AetherOS 路线图

> AetherOS 2.0 是在 v1（C#/WPF）基础上完全重写的 Rust + Tauri + React 版本。

## v2.0 — 架构重写（Rust + Tauri + React）

从 C#/WPF 迁移到模块化 Capability-Provider 平台，分阶段执行。

| 阶段 | 范围 | 状态 |
|---|---|---|
| **Phase 0** | 项目骨架：Cargo workspace、pnpm monorepo、Tauri 壳、React 框架、Tailwind 主题、国际化、CI | ✅ 完成 |
| **Phase 1** | 核心运行时：`aether-core` 模型/trait、`aether-runtime` 注册中心 + 事件总线 + 调度器、IPC 命令层、演示 Provider | ✅ 完成 |
| **Phase 2** | 仪表盘与 UI 框架：侧边栏/顶栏布局、路由、主题切换、双语切换、基础 IPC、真实系统信息仪表盘 | ✅ 完成 |
| **Phase 3** | 系统信息与监控：`aether-system` 硬件信息、`aether-monitor` Provider、实时图表 | ✅ 完成 |
| **Phase 4** | 清理模块：扫描规则、执行管线、进度与结果（全链路） | ✅ 完成 |
| **Phase 5** | 优化与恢复：电源计划、启动项、虚拟内存、还原点、操作历史 | ✅ 完成 |
| **Phase 6** | 插件 SDK 与工程化：SDK、托盘/自启/静默、AI 分析、发布工作流（动态加载延后） | ✅ 完成 |

## v2.x

- 完善插件生态（内置 + 示例规则包）
- 扩充自动化测试（Rust 单元 + 前端组件 + E2E）
- 完善双语文档体系

## 未来

- 更多平台抽象（macOS / Linux 系统引擎）
- 增强诊断与遥测
- 社区扩展市场
