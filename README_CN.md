<p align="center">
  <img src="src-tauri/icons/128x128.png" width="80" alt="TalkType" />
</p>

<h1 align="center">TalkType</h1>

<p align="center">
  <b>别打字了，直接说话。</b><br>
  手机语音输入 → 文字实时出现在电脑光标处 → 任何应用都能用。<br>
  <i>为 Vibe Coding 而生的语音输入工具。</i>
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/puke3615/TalkType?style=flat-square&color=6366f1" alt="Release" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License" /></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=flat-square" alt="Platform" />
  <img src="https://img.shields.io/badge/i18n-English%20%7C%20%E4%B8%AD%E6%96%87-green?style=flat-square" alt="i18n" />
</p>

<p align="center">
  <a href="#-快速开始">快速开始</a> •
  <a href="#-核心特性">核心特性</a> •
  <a href="#-为什么选-talktype">对比</a> •
  <a href="#-下载安装">下载</a> •
  <a href="README.md">English</a>
</p>

---

## ⚡ 比打字快 3 倍

> 为什么要打字？说话就行了。

| | 键盘打字 | 用 TalkType 说话 |
|---|:---:|:---:|
| **速度** | ~40 词/分钟 | **~150 词/分钟** |
| **200 字的 prompt** | ~5 分钟 | **~1.3 分钟** |
| **双手** | 被占用 | 可以自由浏览代码 |
| **疲劳度** | 容易腱鞘炎 | 轻松自然 |

**说话比打字快 3.75 倍。** 一个 200 字的复杂 prompt，打字要 5 分钟，说话只要 80 秒。

## 演示

<!-- TODO: 替换为实际 GIF -->

> 📹 _GIF 即将到来 — 演示：打开应用 → 扫码 → 对手机说话 → 文字出现在 Cursor 中_
>
> **欢迎贡献：** 如果你在使用 TalkType，欢迎录一段 demo 并提交 PR！

## 🚀 快速开始

**30 秒上手：**

1. **下载** → [最新版本](../../releases/latest)（支持 macOS / Windows / Linux）
2. **打开 TalkType** → 电脑上显示二维码
3. **手机扫码** → 浏览器自动打开（手机不需要安装任何 App）
4. **开始说话** → 文字实时出现在电脑光标处 ✨

> ⚠️ 手机和电脑需要连接 **同一个 WiFi**。

### 小技巧

- 📌 **添加到主屏幕** — 在手机浏览器中「添加到主屏幕」，下次免扫码一键打开
- 🌍 **中英双语** — 应用内置中英文切换，自动检测你的语言

## ✨ 核心特性

- 🎙️ **手机原生语音输入** — 使用手机内置语音识别，无需下载任何模型
- ⌨️ **全局文字注入** — 文字直接出现在电脑光标位置，任何应用都能用
- 📱 **手机免安装** — 手机端是网页，扫码即用
- 🔒 **纯局域网通信** — 所有数据只在 WiFi 内传输，不经过任何外部服务器
- 📷 **扫码即连** — 二维码连接，秒级上手
- 🔄 **实时同步** — 说话的同时文字就出现，逐字同步
- 🔁 **智能重连** — 记住连接记录，同一 WiFi 下自动重连
- ⚡ **场景自动化** — 创建自定义工作流（静默触发、按键组合、自定义动作序列）
- 🌍 **国际化** — 中英双语界面，自动检测语言
- 🌍 **三平台支持** — macOS / Windows / Linux
- 📲 **PWA 支持** — 添加到主屏幕后像 App 一样使用

## 🎯 为什么选 TalkType？

### 核心差异

其他语音输入工具要么在本地跑 Whisper（1-3 GB 模型下载，占 CPU/GPU），要么用云端 API（延迟高、要收费、隐私风险）。

TalkType 的思路完全不同：**你的手机就是语音引擎。** 苹果、Google、三星的语音识别是在数十亿数据上训练的，原生支持 100+ 种语言，而且已经在你手机上了。TalkType 只是通过局域网 WiFi 把这个能力桥接到你的电脑上。

**零配置。零成本。隐私零泄露。**

### 与其他工具对比

| | **TalkType** | SuperWhisper | Wispr Flow | VoiceTypr | MacWhisper |
|---|:---:|:---:|:---:|:---:|:---:|
| **价格** | **免费** | $9/月 | $15/月 | 免费 | 付费 |
| **开源** | ✅ | ❌ | ❌ | ✅ | ❌ |
| **语音引擎** | 手机原生 | 本地 Whisper | 云端 AI | 本地 Whisper | 本地 Whisper |
| **模型下载** | **无需** | 1–3 GB | 无 | 1–3 GB | 1–3 GB |
| **上手时间** | **30 秒** | 5–10 分钟 | 5 分钟 | 5–10 分钟 | 5 分钟 |
| **语言支持** | **100+ 原生** | ~50 | ~30 | ~50 | ~50 |
| **macOS** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Windows** | ✅ | ❌ | ❌ | ✅ | ❌ |
| **Linux** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **隐私** | **纯局域网** | 本地 | ☁️ 云端 | 本地 | 本地 |

## 💡 适用场景

| 场景 | 说明 |
|---|---|
| 🧑‍💻 **Vibe Coding** | 对 Cursor / Copilot / Windsurf 说出 prompt，比打字快 3 倍 |
| 🤖 **AI 对话** | 跟 ChatGPT / Claude / Gemini 语音聊天 |
| 📝 **写作** | 写邮件、写文档、做笔记，解放双手 |
| 💬 **聊天** | 在微信 / 钉钉 / Slack / Discord 语音回复 |
| 🔍 **搜索** | 在任何应用或浏览器中语音搜索 |

## 📦 下载安装

前往 [**Releases**](../../releases/latest) 下载对应系统的安装包：

| 系统 | 下载文件 | 说明 |
|---|---|---|
| 🍎 **macOS (Apple Silicon)** | `TalkType_x.x.x_aarch64.dmg` | M1 / M2 / M3 / M4 芯片 |
| 🍎 **macOS (Intel)** | `TalkType_x.x.x_x64.dmg` | 老款 Intel 芯片 |
| 🪟 **Windows** | `TalkType_x.x.x_x64-setup.exe` | Windows 10 / 11 |
| 🐧 **Linux** | `.deb` / `.AppImage` | Ubuntu / Debian 等 |

> 💡 **不知道自己 Mac 是哪种芯片？** 点左上角  → 关于本机 → 看「芯片」一栏。显示 "Apple M1/M2/…" 就下载 `aarch64`，显示 "Intel" 就下载 `x64`。

## ⚙️ 各系统注意事项

<details>
<summary><b>🍎 macOS</b></summary>

- 首次使用需授权 **辅助功能** 权限：
  系统设置 → 隐私与安全性 → 辅助功能 → 开启 TalkType ✅
- 应用会引导你完成授权
- 关闭窗口最小化到 **菜单栏托盘**（右键点击 → 退出）

</details>

<details>
<summary><b>🪟 Windows</b></summary>

- 首次运行时允许 **防火墙** 访问（否则手机无法连接）
- 关闭窗口后在 **系统托盘** 后台运行（右键点击 → 退出）

</details>

<details>
<summary><b>🐧 Linux</b></summary>

- `.deb` 安装包会自动处理依赖
- 使用 `.AppImage` 时先赋予执行权限：`chmod +x TalkType_*.AppImage`

</details>

## ❓ 常见问题

<details>
<summary><b>为什么用手机语音输入而不是本地 Whisper？</b></summary>

手机的语音识别引擎（苹果、Google、三星）是在数十亿数据上训练的，原生支持 100+ 种语言，且针对移动硬件深度优化。不需要下载模型（省 1-3 GB），不需要 GPU，不需要配置 —— 开箱即用，瞬间响应。

</details>

<details>
<summary><b>数据安全吗？</b></summary>

安全。所有通信都通过局域网 WiFi 的 WebSocket 进行。数据永远不会离开你的本地网络。没有云服务器，没有账号系统，没有数据采集。

</details>

<details>
<summary><b>手机扫码后打不开页面？</b></summary>

- 确认手机和电脑在 **同一个 WiFi** 网络
- 检查电脑的防火墙是否阻止了 TalkType 的网络访问
- 尝试关闭 VPN 或代理软件

</details>

<details>
<summary><b>连接成功但电脑没有收到文字？</b></summary>

- **macOS**：检查是否已授权「辅助功能」权限（系统设置 → 隐私与安全性 → 辅助功能）
- 确认电脑端有一个输入框正在聚焦（比如打开记事本，点击输入区域）
- TalkType 将文字输入到当前活跃的光标位置

</details>

## 🔧 工作原理

```
┌─────────────┐       WiFi（局域网）      ┌──────────────┐
│  📱 手机     │  ◄── WebSocket ──►       │  💻 电脑      │
│ （浏览器）   │      实时文字同步         │ （桌面应用）   │
│              │                          │      ↓        │
│  🎤 语音     │                          │  ⌨️ 键盘模拟   │
│  → 文字      │                          │              │
└─────────────┘                           └──────┬───────┘
                                                  ↓
                                           当前活跃应用
                                         (Cursor / Claude /
                                          ChatGPT / 文档)
```

**技术栈：**

- 桌面端：Tauri 2 + Rust（Axum HTTP/WS 服务 + 键盘模拟）
- 手机端：纯 HTML/CSS/JS PWA（由桌面端服务提供，手机免安装）

## 🛠️ 开发者指南

<details>
<summary><b>从源码构建</b></summary>

### 环境要求

- Rust 1.70+（推荐最新 stable）
- Node.js 18+
- Tauri CLI：`cargo install tauri-cli --version "^2"`

### 开发模式

```bash
git clone https://github.com/puke3615/TalkType.git
cd TalkType
npm install
cargo tauri dev
```

### 构建

```bash
cargo tauri build
```

产物在 `src-tauri/target/release/bundle/` 目录下。

### 项目结构

```
TalkType/
├── src/index.html              # 桌面端 UI（国际化、二维码、场景编辑器）
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs              # 应用入口 & Tauri 命令
│   │   ├── server.rs           # HTTP + WebSocket 服务（Axum）
│   │   ├── mobile.html         # 手机端 PWA 页面（国际化、根路径提供）
│   │   ├── keyboard.rs         # 键盘模拟（按平台实现）
│   │   ├── scene.rs            # 场景自动化引擎
│   │   ├── network.rs          # 局域网 IP 发现
│   │   └── qrcode_gen.rs       # 二维码生成
│   └── tauri.conf.json
└── .github/workflows/          # CI/CD（macOS + Windows + Linux）
```

</details>

## 📄 开源协议

[MIT](LICENSE) — 自由使用、修改和分发。

---

<p align="center">
  <b>如果 TalkType 帮到了你，给个 ⭐ Star 吧！</b><br>
  <sub>使用 <a href="https://v2.tauri.app">Tauri 2</a> + Rust 构建 ❤️</sub>
</p>
