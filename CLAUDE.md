# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在本仓库中工作时提供指导。

## 项目概述

openclaw-sensevoice 是一个离线语音转文字 (STT) 的 OpenClaw 插件，用于替代默认的 Whisper 插件。基于阿里巴巴 SenseVoice 模型，通过 ONNX 运行时实现快速、针对中文优化的语音转写。支持中文、英文、日文、韩文和粤语。

## 构建与开发命令

所有 Rust 命令在 `rust/` 目录下执行：

```bash
cargo build --release          # 发布构建（二进制文件：sensevoice-cli）
cargo test                     # 运行测试
cargo fmt --check              # 检查代码格式
cargo clippy -- -D warnings    # 代码检查（所有警告视为错误）
```

交叉编译目标：`x86_64-apple-darwin`、`aarch64-apple-darwin`、`x86_64-unknown-linux-gnu`、`aarch64-unknown-linux-gnu`。

## 架构

**数据流：** 语音消息 → OpenClaw 插件 (TypeScript) → `sensevoice-cli` 二进制文件 (Rust) → 转写文本注入消息

### 组件

- **`index.ts`** — OpenClaw 插件入口。通过 `onMessage` 钩子拦截语音消息，调用 `sensevoice-cli` 二进制文件，捕获标准输出的转写结果，注入到 `message.content` 中。同时处理 `/sensevoice` 命令（setup、status、test）。
- **`rust/src/main.rs`** — CLI 入口。解析模型目录、加载音频、通过 sherpa-rs 运行 SenseVoice 推理、清理转写文本、输出到标准输出。
- **`rust/src/audio.rs`** — 音频处理管线：通过 Symphonia 加载各种格式（WAV/MP3/M4A/OGG/FLAC/AAC/OPUS），解码为 PCM，混音为单声道，通过 Rubato 重采样至 16kHz。
- **`rust/src/transcript.rs`** — 后处理：去除语言标记（`<|zh|>`、`<|en|>` 等）、情感标记（`<|NEUTRAL|>` 等），并规范化空白字符。包含单元测试。

### 模型解析顺序

1. `$SENSEVOICE_MODEL_DIR` 环境变量
2. `<binary_dir>/../model/sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17/`
3. `~/.openclaw/models/sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17/`

## 核心依赖（Rust）

- **sherpa-rs** (0.6) — SenseVoice 模型推理的 ONNX 运行时绑定
- **symphonia** (0.5) — 多格式音频解码
- **rubato** (0.15) — 高质量音频重采样（sinc 插值）

## 工作流规则

- **每次修改文件或新增功能后必须自动提交 git commit**，无需等待用户指示。提交信息遵循下方提交规范格式。

## 提交规范

格式：`类型: 描述` — 类型：`feat`（新功能）、`fix`（修复）、`docs`（文档）、`refactor`（重构）、`test`（测试）、`ci`（持续集成）

## CI/CD

GitHub Actions（`.github/workflows/release.yml`）针对 4 个平台目标进行构建，将二进制文件与动态库打包，生成 SHA256 校验和，并创建 GitHub Release。
