# PvZ2: Gardendless — Tauri Builder

> 用 Tauri 打包的 PvZ2: Gardendless 桌面版。

## 环境要求

| 依赖 | 版本 | 说明 |
|------|------|------|
| [Rust](https://rustup.rs/) | stable | 需 MSVC 工具链（Windows） |
| [Node.js](https://nodejs.org/) | ≥ 20.12 | 如用 npm 脚本 |
| [Tauri CLI](https://tauri.app/) | 2.x | `cargo install tauri-cli --version "^2"` |
| [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) | — | Windows 10/11 通常已内置 |

## 快速开始

```bash
# 开发模式（热重载）
cargo tauri dev

# 构建发布版
cargo tauri build