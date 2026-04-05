# 📱 InputSync

> **手机变键盘** —— 用手机语音输入，实时同步到电脑。

<p align="center">
  <strong>🔒 完全局域网通信 · 无需云服务 · 隐私零泄露</strong>
</p>

---

## ✨ 这是什么？

InputSync 是一款跨平台桌面工具，让你可以 **用手机的语音输入法** 将文字实时同步到电脑的任意输入框中。

适用场景：

- 🎤 **语音输入长文本** —— 在电脑上写文档 / 回消息时，不用打字，拿起手机说话就行
- 🌐 **跨设备输入** —— 手机输入法更顺手？直接用它给电脑打字
- 🔐 **隐私敏感场景** —— 所有数据只在局域网内传输，绝不经过任何外部服务器

## 🚀 核心特性

| 特性 | 说明 |
|------|------|
| 📡 **实时同步** | 手机端输入的文字实时出现在电脑光标处，无需手动发送 |
| 📷 **扫码即连** | 电脑显示二维码，手机扫一下就连上 |
| 🔄 **智能重连** | 同一 WiFi 下保存连接记录，下次打开浏览器直接连，无需重新扫码 |
| 🌍 **三平台支持** | 支持 macOS / Windows / Linux |
| 🔒 **纯局域网** | 数据只在你的 WiFi 内传输，不经过任何云服务 |
| 📱 **手机免安装** | 手机端是网页，不需要装任何 App |

## 📦 下载安装

前往 [**Releases**](../../releases) 页面下载对应系统的安装包：

| 系统 | 下载文件 | 说明 |
|------|---------|------|
| 🍎 **macOS (Apple Silicon)** | `InputSync_x.x.x_aarch64.dmg` | M1 / M2 / M3 / M4 芯片的 Mac |
| 🍎 **macOS (Intel)** | `InputSync_x.x.x_x64.dmg` | 老款 Intel 芯片的 Mac |
| 🪟 **Windows** | `InputSync_x.x.x_x64-setup.exe` | 64 位 Windows 10/11 |
| 🐧 **Linux** | `InputSync_x.x.x_amd64.deb` / `.AppImage` | Ubuntu / Debian 或通用 Linux |

> 💡 **不知道自己 Mac 是哪种芯片？** 点左上角  → 关于本机 → 看「芯片」一栏。显示 "Apple M1/M2/..." 就下载 `aarch64`，显示 "Intel" 就下载 `x64`。

## 🎯 使用方法

### 第一步：启动应用

双击打开 InputSync，等待几秒钟，应用窗口会显示一个 **二维码** 和连接地址。

### 第二步：手机扫码连接

拿起手机，打开 **相机** 或任意扫码工具，扫描电脑上的二维码。

> ⚠️ **重要**：手机和电脑必须连接 **同一个 WiFi** 网络！

扫码后手机浏览器会自动打开 InputSync 的输入页面，顶部显示 **"已连接"** 即表示连接成功 🟢。

### 第三步：开始输入

在手机页面的输入框中 **打字或使用语音输入**，文字会 **实时同步** 到电脑当前光标所在的位置（比如你正在编辑的文档、聊天窗口、搜索框等）。

- 输入文字 → 自动同步到电脑
- 点「↵ 回车」→ 在电脑端按下回车键
- 点「清空」→ 清空手机和电脑端的输入内容

### 🔁 下次使用（免扫码）

InputSync 会自动保存连接记录。下次只要在 **同一个 WiFi** 下：

1. 电脑上打开 InputSync
2. 手机浏览器打开 **上次的页面**（历史记录 / 书签）即可直接连接

> 💡 **更快捷的方式**：在手机浏览器中把页面「**添加到主屏幕**」，以后像打开 App 一样一键打开！

## ⚙️ 各系统注意事项

### 🍎 macOS

- **首次使用需授权「辅助功能」权限**，否则无法将文字输入到其他应用：
  - 系统设置 → 隐私与安全性 → 辅助功能 → 开启 InputSync ✅
  - 应用内也会有引导提示
- 关闭窗口不会退出程序，会最小化到 **菜单栏托盘**
- 右键菜单栏托盘图标 → 「退出」可完全退出

### 🪟 Windows

- 首次运行时，Windows 防火墙可能弹窗询问网络访问权限，请 **选择允许**（否则手机无法连接）
- 关闭窗口后会在 **系统托盘** 中后台运行
- 右键托盘图标 → 「退出」可完全退出

### 🐧 Linux

- 需要确保系统已安装 `webkit2gtk` 等依赖（`.deb` 安装包会自动处理）
- 使用 `.AppImage` 时需先赋予执行权限：`chmod +x InputSync_*.AppImage`

## ❓ 常见问题

<details>
<summary><b>📱 手机扫码后打不开页面？</b></summary>

- 确认手机和电脑在 **同一个 WiFi** 网络
- 检查电脑的防火墙是否阻止了 InputSync 的网络访问
- 尝试关闭 VPN 或代理软件

</details>

<details>
<summary><b>⌨️ 连接成功但电脑没有收到文字？</b></summary>

- **macOS**：请检查是否已授权「辅助功能」权限（系统设置 → 隐私与安全性 → 辅助功能）
- 确认电脑端有一个 **输入框正在聚焦**（比如打开记事本，点击输入区域）
- InputSync 将文字输入到 **当前活跃的输入光标位置**

</details>

<details>
<summary><b>🔌 连接经常断开？</b></summary>

- 检查 WiFi 信号是否稳定
- InputSync 有自动重连机制，短暂断开后会自动恢复
- 如果手机锁屏或切换到其他 App，连接可能暂时中断，回到页面后会自动重连

</details>

<details>
<summary><b>🔢 默认端口被占用了怎么办？</b></summary>

InputSync 默认使用端口 `5678`。如果被占用，目前需要修改源码中的端口号后重新编译。

</details>

## 🏗️ 技术架构

```
┌─────────────┐         WiFi / LAN          ┌──────────────┐
│  📱 手机端   │ ◄──── WebSocket ────►       │  💻 电脑端    │
│  (浏览器)    │      实时文字同步            │  (桌面应用)   │
└─────────────┘                              └──────┬───────┘
                                                    │
                                              模拟键盘输入
                                                    ▼
                                            当前聚焦的应用
                                          (文档/聊天/搜索...)
```

- **桌面端**：Tauri 2 + Rust（HTTP/WebSocket 服务 + 键盘模拟）
- **手机端**：纯 HTML/CSS/JS 网页（由桌面端服务提供，手机免安装）

## 🛠️ 开发者指南

如果你想参与开发或二次修改，请参考以下步骤：

<details>
<summary><b>展开查看开发环境搭建</b></summary>

### 环境要求

- Rust 1.70+（推荐最新 stable）
- Node.js 18+
- Tauri CLI：`cargo install tauri-cli --version "^2"`

### 快速开始

```bash
git clone https://github.com/yourname/InputSync.git
cd InputSync
npm install
cargo tauri dev
```

### 项目结构

```
InputSync/
├── src/index.html            # 桌面端 UI
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs            # 应用入口 & Tauri 命令
│   │   ├── server.rs         # HTTP + WebSocket 服务
│   │   ├── mobile.html       # 手机端页面
│   │   ├── keyboard.rs       # 键盘模拟（macOS/Windows/Linux）
│   │   ├── network.rs        # 局域网 IP 发现
│   │   └── qrcode_gen.rs     # 二维码生成
│   └── tauri.conf.json       # Tauri 配置
└── .github/workflows/        # CI/CD 自动构建
```

### 构建

```bash
cargo tauri build
```

产物在 `src-tauri/target/release/bundle/` 目录下。

</details>

## 📄 开源协议

[MIT License](LICENSE) — 自由使用、修改和分发。

---

<p align="center">
  如果觉得有用，欢迎给个 ⭐ Star！
</p>
