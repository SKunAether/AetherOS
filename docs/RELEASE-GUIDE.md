# AetherOS 2.0 Release Guide

## Local validation

```bash
cd apps/desktop/src-tauri
cargo check --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
```

## Build release assets (installer exe)

```bash
cd apps/desktop/src-tauri
../frontend/node_modules/.bin/tauri build
```

Outputs:

- `target/release/aetheros-desktop.exe` — standalone executable
- `target/release/bundle/nsis/AetherOS Guardian_<version>_x64-setup.exe` — NSIS installer

## Generate update manifest

The installer's version is read from `apps/desktop/src-tauri/tauri.conf.json`.
To generate the auto-update manifest (`installer/update-manifest.json`):

```powershell
$version = (Get-Content apps/desktop/src-tauri/tauri.conf.json | ConvertFrom-Json).version
$setup = Get-ChildItem "target/release/bundle/nsis" -Filter "*.exe" | Select-Object -First 1
$sha = (Get-FileHash $setup.FullName -Algorithm SHA256).Hash.ToLower()
$manifest = [ordered]@{
  version = $version
  notes = "AetherOS Guardian $version"
  pub_date = (Get-Date).ToUniversalTime().ToString("o")
  platforms = @{
    "windows-x86_64" = @{ signature = ""; url = "https://github.com/SKunAether/AetherOS/releases/latest/download/$($setup.Name)" }
  }
}
$manifest | ConvertTo-Json -Depth 5 | Set-Content installer/update-manifest.json -Encoding UTF8
```

## Automatic GitHub Actions release

Push a version tag matching `v*`:

```bash
git tag v2.0.0
git push origin v2.0.0
```

The `release.yml` workflow builds, packages and uploads:

- `AetherOS Guardian_<version>_x64-setup.exe`
- `update-manifest.json`

## Update flow

```text
Old client
→ latest/download/update-manifest.json
→ version comparison
→ installer download
→ SHA-256 verification
→ run installer
→ restart
```

> Note: full silent auto-update is planned; current version ships with the "检查更新"
> entry that opens the releases page.
