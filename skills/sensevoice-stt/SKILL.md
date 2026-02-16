---
name: sensevoice-stt
description: Offline speech-to-text transcription using SenseVoice. Fast (~1s cold start), lightweight (287MB), supports Chinese, English, Japanese, Korean and Cantonese. Rust/ONNX powered alternative to Whisper.
tags: stt, speech-to-text, sensevoice, offline, chinese, cantonese, audio, transcription, voice
---

# SenseVoice Speech-to-Text

离线语音转文字插件，基于阿里巴巴 SenseVoice 模型 + Rust/ONNX Runtime。

## 特点
- 冷启动 ~1 秒（Whisper 需 30-40 秒）
- 总体积 287MB（Whisper ~1.5GB）
- 无常驻内存占用
- 中文转写质量优化
- 原生粤语支持

## 使用
语音消息自动转写，或使用命令：
- `/sensevoice setup` — 下载模型并配置
- `/sensevoice status` — 检查安装状态
- `/sensevoice test <文件>` — 手动测试转写
