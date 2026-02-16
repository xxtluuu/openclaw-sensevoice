[中文](README.md) | English

# ⚡ openclaw-sensevoice

**Offline speech-to-text plugin for OpenClaw — fast, accurate, lightweight, private**

Receive a voice message, get text in 1 second. Excellent Chinese recognition accuracy, supports Chinese/English/Japanese/Korean/Cantonese, model only 228MB, runs completely offline.

## 🤔 Why Replace the Official Voice Plugin?

The official OpenClaw voice plugin is based on OpenAI Whisper, designed for international users and covering 100+ languages. But for Chinese users, it has an awkward **dilemma**:

> **Either slow, or memory-hungry — and Chinese recognition still isn't great.**

- ❌ **Cold start mode**: Each voice message takes **30~40 seconds** to transcribe, severely disrupting the chat experience
- ❌ **Resident memory mode**: Startup speed improves, but permanently occupies **~1.6GB of memory**, a heavy burden for users deploying on VPS
- ❌ **Chinese recognition**: Whisper is designed to balance 100+ languages, Chinese is not its strength, and Cantonese is not supported at all

**SenseVoice solves all problems at once**: model + runtime totals only **287MB**, each cold start takes just **~1 second** to complete transcription — fast enough that there's no need to keep it resident in memory. Send a voice message, and the reply appears almost instantly, as natural as typing.

| | Official Plugin (Whisper) | ⚡ SenseVoice |
|---|---|---|
| **Wait time per message** | Cold start 30~40s | **~1 second** |
| **Model + runtime size** | ~1.5GB | **287MB** |
| **Needs resident memory?** | Yes (otherwise too slow) | **No (cold start is fast enough)** |
| **Chinese recognition quality** | Average (internationalization first) | **Optimized for Chinese** |
| **Cantonese support** | ❌ | ✅ Native support |
| **Inference engine** | Python / PyTorch | **Rust + ONNX Runtime** |
| **Privacy** | ✅ Offline | ✅ Offline |

## ✨ Highlights

- 🚀 **Sub-second response** — Cold start to output in ~1 second, no need for resident memory, voice messaging is as smooth as typing
- 🌏 **Five-language transcription** — Chinese, English, Japanese, Korean, Cantonese, with automatic language detection
- 🧠 **Ultra-lightweight** — INT8 quantized model only 228MB, full installation under 300MB, 1/5 the size of Whisper
- 🦀 **Native Rust** — Compiled to native binary + ONNX Runtime, no Python dependency, zero startup overhead
- 🔒 **Completely offline** — Audio data never leaves the device, zero privacy risk
- 📦 **One-click install** — Automatically downloads model, configures OpenClaw, ready to use out of the box

## 🚀 Quick Start

### Method 1: OpenClaw Plugin Install (Recommended)

```bash
openclaw plugins install https://github.com/falebao/openclaw-sensevoice/releases/download/v0.2.0/openclaw-sensevoice-v0.2.0.tar.gz
```

After installation, run setup once to automatically download the model and configure:

```
/sensevoice setup
```

**Once setup is complete, just send voice messages — the plugin automatically transcribes the audio and AI replies based on the transcribed text. No extra steps needed.**

### Method 2: One-line Script

```bash
curl -fsSL https://raw.githubusercontent.com/falebao/openclaw-sensevoice/main/install.sh | bash
```

Automatically detects platform, downloads pre-compiled binary and model to `~/.openclaw/`.

### Method 3: Build from Source

```bash
git clone https://github.com/falebao/openclaw-sensevoice.git
cd openclaw-sensevoice/rust
cargo build --release
```

## 🎙️ How It Works

After installation and setup, voice transcription is fully automatic:

```
You send a voice message → Plugin auto-transcribes (~1s) → AI replies based on transcribed content
```

No manual commands needed. Sending voice is just like typing — AI directly understands what you said and responds.

## 📊 Performance Benchmarks

Test environment: Apple Silicon (M series), macOS, CPU-only inference, including model loading.

| Language | Audio Duration | Transcription Time | Real-time Factor |
|----------|---------------|-------------------|-----------------|
| 🇨🇳 Chinese | 5.6s | 1.09s | **5.1x** |
| 🇺🇸 English | 7.2s | 1.16s | **6.2x** |
| 🇯🇵 Japanese | 7.2s | 1.15s | **6.2x** |
| 🇰🇷 Korean | 4.6s | 1.06s | **4.3x** |
| 🇭🇰 Cantonese | 5.1s | 1.10s | **4.7x** |

> The times above include the full pipeline: model loading → audio decoding → inference → output. In daily use, system file caching makes subsequent calls even faster.

## 🛠️ Plugin Commands

| Command | Description |
|---------|-------------|
| `/sensevoice setup` | Download model (~228MB) and auto-configure openclaw.json |
| `/sensevoice status` | Check binary, model, and configuration status |
| `/sensevoice test <file>` | Test transcription on a specified audio file |

## 💻 Supported Platforms

| OS | Architecture | Notes |
|----|-------------|-------|
| macOS | ARM64 | Apple Silicon (M1/M2/M3/M4) |
| macOS | x86_64 | Intel Mac |
| Linux | x86_64 | Server / Desktop |
| Linux | ARM64 | Raspberry Pi / ARM Server |

## 🦀 Why Rust Instead of Python?

The official Whisper plugin's tech stack is Python + PyTorch. While the Python ecosystem is mature, it has inherent drawbacks as a **background service**:

| | Python (Whisper) | Rust (SenseVoice) |
|---|---|---|
| **Cold start** | Slow — needs to load Python interpreter → import dozens of modules → load model | **Fast** — native binary executes directly, no interpreter overhead |
| **Runtime deps** | Python 3.x + pip + PyTorch + many packages | **Zero deps** — single binary + 3 dynamic libraries, ready to use |
| **Memory usage** | Python interpreter + GC + PyTorch = high memory baseline | **Precise memory control** — no GC, released when done |
| **Distribution** | Need to manage virtual environments, package version conflicts | **One tar.gz, extract and run** — doesn't pollute the system |
| **Cross-platform** | PyTorch installation experience varies greatly across platforms | **Pre-compiled binaries** — CI auto-builds for 4 platforms |

> 💡 In short: **The Rust approach makes speech-to-text as simple as "calling a command-line tool"**. No need to install Python, no need to manage virtual environments, no need to worry about package conflicts — download, extract, run, done.

## 📝 Technical Details

**Model search order**:
1. `$SENSEVOICE_MODEL_DIR` environment variable
2. `<binary-dir>/../model/` (plugin layout)
3. `~/.openclaw/models/` (standard location)

**Installation size**: Binary + dynamic libraries ~59MB, model ~228MB, total approximately 287MB.

**macOS users**: Unsigned binaries need Gatekeeper quarantine removed, the install script handles this automatically (`xattr -cr`).

## 💡 Inspiration

I used to voice-dictate everything with Claude Code on desktop — it was incredibly smooth. But when I switched to OpenClaw on mobile via IM, the voice experience was surprisingly poor — slow transcription, long waits, a completely different story.

As the developer of [sayup.ai](https://www.sayup.ai), porting sayup's core engine over was the most natural thing to do — same tech stack, same blazing speed.

If you want the same smooth voice input experience on desktop, check out the macOS client: **[www.sayup.ai](https://www.sayup.ai)** (Windows support is experimental — limited testing devices available).

Follow me on X for updates: [@luuu2024](https://x.com/luuu2024)

## 📄 License

MIT
