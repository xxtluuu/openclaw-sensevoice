---
name: transcribe
description: Transcribe audio files to text using SenseVoice offline STT. Use when the user wants to convert speech/audio to text, transcribe voice memos, or process audio files. Supports WAV, MP3, M4A, OGG, FLAC, AAC, OPUS in Chinese, English, Japanese, Korean, and Cantonese.
disable-model-invocation: false
allowed-tools: Bash(sensevoice-cli *), Bash(~/.openclaw/bin/sensevoice-cli *), Bash(*/sensevoice-cli *), Read
---

# Audio Transcription with SenseVoice

Transcribe audio files using the local sensevoice-cli binary.

## Binary locations (try in order)
1. `~/.openclaw/bin/sensevoice-cli`
2. `./rust/target/release/sensevoice-cli`

## Usage
```bash
sensevoice-cli <audio_file_path>
```

Output is plain text transcription to stdout.

## Steps
1. Locate the audio file path (ask user if not provided)
2. Find the sensevoice-cli binary (check both paths above)
3. Run: `<binary_path> <audio_file_path>`
4. Return the transcription text to the user
5. For multiple files, process sequentially and combine results

## Supported formats
WAV, MP3, M4A, OGG, FLAC, AAC, OPUS

## If binary not found
- Build from source: `cd rust && cargo build --release`
- Or install: `curl -fsSL https://raw.githubusercontent.com/xxtluuu/openclaw-sensevoice/main/install.sh | bash`

## If model not found
Model should be at: `~/.openclaw/models/sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17/`
Download via install.sh or `/sensevoice setup` command.
