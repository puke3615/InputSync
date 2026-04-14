<p align="center">
  <img src="src-tauri/icons/128x128.png" width="80" alt="TalkType" />
</p>

<h1 align="center">TalkType</h1>

<p align="center">
  <b>Stop typing prompts. Just speak.</b><br>
  Your phone's voice input → instant text at your computer's cursor → in any app.<br>
  <i>Global voice input for vibe coding.</i>
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/puke3615/TalkType?style=flat-square&color=6366f1" alt="Release" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License" /></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=flat-square" alt="Platform" />
  <img src="https://img.shields.io/badge/i18n-English%20%7C%20%E4%B8%AD%E6%96%87-green?style=flat-square" alt="i18n" />
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-why-talktype">Comparison</a> •
  <a href="#-download">Download</a>
</p>

<p align="center">
  <b>English</b> | <a href="README_CN.md">中文</a>
</p>

---

## ⚡ 3x Faster Than Typing

> Why type when you can talk?

| | Typing | Speaking with TalkType |
|---|:---:|:---:|
| **Speed** | ~40 WPM | **~150 WPM** |
| **200-word prompt** | ~5 minutes | **~1.3 minutes** |
| **Hands** | Both occupied | Free to browse code |
| **Fatigue** | RSI risk | Relaxed |

**Speaking is 3.75x faster than typing.** A complex 200-word prompt that takes 5 minutes to type? Speak it in under 80 seconds.

## Demo

<!-- TODO: Replace with actual GIF -->

> 📹 _GIF coming soon — showing: open app → scan QR → speak on phone → text appears in Cursor_
>
> **Help wanted:** If you use TalkType, record a short demo and submit a PR!

## 🚀 Quick Start

**Get started in 30 seconds:**

1. **Download** → [Latest Release](../../releases/latest) (macOS / Windows / Linux)
2. **Open TalkType** on your computer → QR code appears
3. **Scan** the QR code with your phone → web page opens (no app install)
4. **Speak** on your phone → tap **Send** → text appears at your cursor ✨

> ⚠️ Phone and computer must be on the **same WiFi**.

### Pro Tips

- 📌 **Add to Home Screen** — on your phone's browser, "Add to Home Screen" for instant access (no rescan needed next time)
- 🌍 **Bilingual UI** — switch between English and Chinese in the app settings

## ✨ Features

- 🎙️ **Phone-native voice input** — uses your phone's built-in STT, no model download needed
- ⌨️ **Global text injection** — text appears at your cursor in ANY app
- 📱 **Zero install on phone** — phone client is a web page, no app needed
- 🔒 **100% local network** — all data stays on your WiFi, zero cloud dependency
- 📷 **QR code pairing** — scan and connect in seconds
- ↵ **Auto-Enter** — optionally press Enter after sending, perfect for chat apps
- 🔁 **Auto reconnect** — remembers connections, auto-reconnects on same WiFi
- 🌍 **i18n** — English and Chinese UI, auto-detects your language
- 🌍 **Cross-platform** — macOS, Windows, Linux
- 📲 **PWA support** — add to phone home screen for app-like experience

## 🎯 Why TalkType?

### The Unfair Advantage

Other voice input tools run Whisper locally (1–3 GB model download, CPU/GPU intensive) or use cloud APIs (latency, cost, privacy risk).

TalkType takes a fundamentally different approach: **your phone IS the voice engine.** Apple's, Google's, and Samsung's STT are trained on billions of data points, support 100+ languages, and are already on your device. TalkType simply bridges that to your computer over local WiFi.

**Zero config. Zero cost. Zero compromise on privacy.**

### vs. Other Voice Input Tools

| | **TalkType** | SuperWhisper | Wispr Flow | VoiceTypr | MacWhisper |
|---|:---:|:---:|:---:|:---:|:---:|
| **Price** | **Free** | $9/mo | $15/mo | Free | Freemium |
| **Open Source** | ✅ | ❌ | ❌ | ✅ | ❌ |
| **Voice Engine** | Phone native | Local Whisper | Cloud AI | Local Whisper | Local Whisper |
| **Model Download** | **None** | 1–3 GB | N/A | 1–3 GB | 1–3 GB |
| **Setup Time** | **30 sec** | 5–10 min | 5 min | 5–10 min | 5 min |
| **Languages** | **100+ native** | ~50 | ~30 | ~50 | ~50 |
| **macOS** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Windows** | ✅ | ❌ | ❌ | ✅ | ❌ |
| **Linux** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Privacy** | **LAN only** | Local | ☁️ Cloud | Local | Local |

## 💡 Use Cases

| Scenario | Description |
|---|---|
| 🧑‍💻 **Vibe Coding** | Speak prompts to Cursor, Copilot, Windsurf, or any AI coding tool |
| 🤖 **AI Chat** | Dictate to ChatGPT, Claude, Gemini — 3x faster than typing |
| 📝 **Writing** | Draft emails, docs, and notes hands-free |
| 💬 **Messaging** | Reply in Slack, Discord, WeChat, Teams by voice |
| 🔍 **Search** | Voice search in any app or browser |

## 📦 Download

Go to [**Releases**](../../releases/latest) and download for your platform:

| Platform | File | Notes |
|---|---|---|
| 🍎 **macOS (Apple Silicon)** | `TalkType_x.x.x_aarch64.dmg` | M1 / M2 / M3 / M4 |
| 🍎 **macOS (Intel)** | `TalkType_x.x.x_x64.dmg` | Older Intel Macs |
| 🪟 **Windows** | `TalkType_x.x.x_x64-setup.exe` | Windows 10 / 11 |
| 🐧 **Linux** | `.deb` / `.AppImage` | Ubuntu, Debian, etc. |

> 💡 **Which Mac?** Click  → About This Mac → check "Chip". Apple M1/M2/… = `aarch64`, Intel = `x64`.

## ⚙️ Platform Notes

<details>
<summary><b>🍎 macOS</b></summary>

- Grant **Accessibility** permission on first launch:
  System Settings → Privacy & Security → Accessibility → Enable TalkType ✅
- The app guides you through this on first run
- Closing the window minimizes to **menu bar tray** (right-click → Quit)

</details>

<details>
<summary><b>🪟 Windows</b></summary>

- Allow **firewall access** when prompted (required for phone to connect)
- Closing the window minimizes to **system tray** (right-click → Quit)

</details>

<details>
<summary><b>🐧 Linux</b></summary>

- `.deb` package handles dependencies automatically
- For `.AppImage`: run `chmod +x TalkType_*.AppImage` first

</details>

## ❓ FAQ

<details>
<summary><b>Why use phone voice input instead of local Whisper?</b></summary>

Phone STT engines (Apple, Google, Samsung) are trained on billions of data points, support 100+ languages natively, and are heavily optimized for mobile hardware. No model download (1–3 GB saved), no GPU required, no configuration — it just works, instantly.

</details>

<details>
<summary><b>Is my data safe?</b></summary>

Yes. All communication happens over your local WiFi via WebSocket. No data ever leaves your network. No cloud, no accounts, no telemetry.

</details>

<details>
<summary><b>Phone can't open the page after scanning?</b></summary>

- Ensure phone and computer are on the **same WiFi**
- Check your computer's firewall settings
- Try disabling VPN or proxy

</details>

<details>
<summary><b>Connected but no text appearing?</b></summary>

- **macOS**: Check Accessibility permission (System Settings → Privacy & Security → Accessibility)
- Make sure an input field is focused on your computer
- Text goes to wherever your cursor currently is

</details>

## 🔧 How It Works

```
┌─────────────┐       WiFi (LAN)        ┌──────────────┐
│  📱 Phone    │  ◄── WebSocket ──►      │  💻 Computer  │
│  (Browser)   │      tap to send        │  (Desktop App)│
│              │                         │       ↓       │
│  🎤 Voice    │                         │  ⌨️ Keyboard  │
│  → Text      │                         │   Injection   │
└─────────────┘                          └───────┬───────┘
                                                 ↓
                                          Any Active App
                                        (Cursor / Claude /
                                         ChatGPT / Docs)
```

**Tech Stack:**

- Desktop: Tauri 2 + Rust (Axum HTTP/WS server + keyboard simulation)
- Mobile: Pure HTML/CSS/JS PWA (served by desktop app, zero install)

## 🛠️ Development

<details>
<summary><b>Build from source</b></summary>

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Node.js 18+
- Tauri CLI: `cargo install tauri-cli --version "^2"`

### Dev Mode

```bash
git clone https://github.com/puke3615/TalkType.git
cd TalkType
npm install
cargo tauri dev
```

### Build

```bash
cargo tauri build
```

Output: `src-tauri/target/release/bundle/`

### Project Structure

```
TalkType/
├── src/index.html              # Desktop UI (i18n, QR code)
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs              # App entry & Tauri commands
│   │   ├── server.rs           # HTTP + WebSocket server (Axum)
│   │   ├── mobile.html         # Mobile PWA page (i18n, served at /)
│   │   ├── keyboard.rs         # Keyboard simulation (per-platform)
│   │   ├── network.rs          # LAN IP discovery
│   │   └── qrcode_gen.rs       # QR code generation
│   └── tauri.conf.json
└── .github/workflows/          # CI/CD (macOS + Windows + Linux)
```

</details>

## 📄 License

[MIT](LICENSE) — free to use, modify, and distribute.

---

<p align="center">
  <b>If TalkType saves you time, give it a ⭐!</b><br>
  <sub>Built with ❤️ using <a href="https://v2.tauri.app">Tauri 2</a> + Rust</sub>
</p>
