# Windows 安装器

AetherOS 2.0 使用 Tauri 内置的 NSIS 安装器（`pnpm tauri build` 自动生成），
不再依赖 Inno Setup。本目录用于存放 Windows 平台相关的安装辅助脚本与说明。

## 生成安装包

```bash
cd apps/desktop/frontend
pnpm tauri build
```

产物位于 `apps/desktop/src-tauri/target/release/bundle/nsis/*.exe`。

## 更新清单

`../update-manifest.json` 由发布工作流（`.github/workflows/release.yml`）自动生成，
客户端通过 Tauri updater 校验版本与签名。
