# Contributing to AetherOS Guardian

Thank you for your interest in contributing to **AetherOS Guardian**! We welcome all forms of contributions — bug reports, feature suggestions, documentation improvements, and code pull requests.

Please read and abide by our [Code of Conduct](CODE_OF_CONDUCT.md) when participating in this project.

---

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please **search existing issues** to avoid duplicates. When you submit a bug report, include:

- **A clear and descriptive title**.
- **Steps to reproduce** the issue.
- **Expected behavior** vs. **actual behavior**.
- **Screenshots or code snippets** (if applicable).
- **Environment details**:
  - Windows version (10/11)
  - Rust version (`rustc --version`)
  - Node.js version (`node --version`)
  - pnpm version (`pnpm --version`)

### Suggesting Enhancements

We welcome feature suggestions. Please provide:

- **A clear and descriptive title**.
- **A detailed description** of the feature and its use case.
- **Any implementation ideas** you have (optional but helpful).

### Pull Requests (Code Contributions)

We follow a standard GitHub flow. Please:

1. **Fork** the repository and create your branch from `main`.
   - Use a descriptive branch name, e.g., `feature/add-disk-cleaner` or `fix/tray-icon-bug`.
2. **Follow the existing coding style**:
   - **Rust**: Use `rustfmt` (run `cargo fmt` before committing).
   - **TypeScript/React**: Use Prettier and ESLint (run `pnpm prettier --write` and `pnpm eslint --fix`).
   - Follow the project’s naming conventions and file structure.
3. **Write clear, concise commit messages** that explain *why* the change is made.
4. **Test your changes thoroughly**:
   - Run `cargo check --workspace` and `cargo clippy --workspace --all-targets` to ensure no lint errors.
   - Run `cargo test --workspace` to pass all unit tests.
   - Manually test the Tauri application (`pnpm tauri dev` in `apps/desktop/frontend`).
5. **Ensure the build passes** – CI will run automatically on your PR.
6. **Submit a Pull Request** to the `main` branch.
   - Clearly describe what your PR does and reference any related issues.

---

## Development Setup

### Prerequisites

- **Windows 10 or 11** (development is Windows-only at this stage)
- **Rust 1.80+** with the MSVC toolchain (install via [rustup](https://rustup.rs/))
- **Node.js 20+** and **pnpm** (install via `npm install -g pnpm`)
- **WebView2** (bundled with Windows 11; for Windows 10, install from Microsoft)
- **NSIS** (for building the installer – required only if you build the full installer locally; CI handles it automatically)

### Clone and Install Dependencies

```bash
git clone https://github.com/SKunAether/AetherOS.git
cd AetherOS
pnpm install                         # install frontend dependencies
Run in Development Mode
bash
cd apps/desktop/frontend
pnpm tauri dev
This launches the React dev server and opens the Tauri window.

Run Linters and Tests
From the project root:

bash
# Rust checks
cargo check --workspace
cargo clippy --workspace --all-targets
cargo test --workspace

# Frontend checks (from apps/desktop/frontend)
pnpm eslint .
pnpm prettier --check .

# Format code
cargo fmt --all
pnpm prettier --write .
Build the Installer Locally
bash
cd apps/desktop/src-tauri
cargo tauri build
The installer will be output to target/release/bundle/nsis/*.exe.

Licensing
By contributing to this project, you agree that your contributions will be licensed under the same GNU Affero General Public License v3.0 (AGPL-3.0) as the project. See the LICENSE file for details.

Code of Conduct
All contributors are expected to adhere to our Code of Conduct. Please report any unacceptable behavior to the project maintainers.

Questions?
If you have any questions about contributing, feel free to open a discussion or reach out to the maintainers via GitHub Issues.

Thank you for helping make AetherOS Guardian better! 🚀
