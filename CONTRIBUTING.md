# Contributing to openclaw-sensevoice

感谢你对 openclaw-sensevoice 的关注！欢迎提交 Issue 和 Pull Request。

## 开发环境

### 前置要求

- [Rust](https://rustup.rs/) 1.70+
- macOS 或 Linux

### 构建

```bash
git clone https://github.com/xxtluuu/openclaw-sensevoice.git
cd openclaw-sensevoice/rust
cargo build --release
```

### 运行测试

```bash
cd rust
cargo test
```

### 代码检查

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## 项目结构

```
openclaw-sensevoice/
├── rust/                  # Rust CLI 核心
│   └── src/
│       ├── main.rs        # 入口
│       ├── audio.rs       # 音频处理
│       └── transcript.rs  # 转写逻辑
├── index.ts               # OpenClaw 插件入口
├── install.sh             # 一键安装脚本
└── openclaw.plugin.json   # 插件配置
```

## Pull Request 规范

1. **Fork** 本仓库并创建功能分支
2. 确保 `cargo fmt` 和 `cargo clippy` 通过
3. 确保 `cargo test` 全部通过
4. Commit 信息格式：`类型: 简要描述`
   - 类型：`feat`（新功能）、`fix`（修复）、`docs`（文档）、`refactor`（重构）、`test`（测试）、`ci`（CI）
   - 示例：`feat: add wav format support`
5. 提交 PR 并描述你的改动

## 报告问题

请通过 [GitHub Issues](https://github.com/xxtluuu/openclaw-sensevoice/issues) 提交，并尽量包含：

- 问题描述
- 复现步骤
- 运行环境（OS、架构）
- 相关日志
