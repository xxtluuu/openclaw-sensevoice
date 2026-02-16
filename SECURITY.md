# Security Policy

## 支持版本

| 版本 | 支持状态 |
|------|---------|
| 最新 release | ✅ 支持 |
| 更早版本 | ❌ 不再维护 |

## 报告漏洞

如果你发现了安全漏洞，请通过以下方式报告：

1. **GitHub Issues**：在 [Issues](https://github.com/falebao/openclaw-sensevoice/issues) 中提交，标注 `security` 标签
2. 请勿在公开 Issue 中包含可被直接利用的漏洞细节

我们会在收到报告后尽快响应并修复。

## 安全设计

- 所有音频处理在本地完成，不上传任何数据
- 安装脚本支持 SHA256 校验，确保二进制完整性
- 不收集任何用户数据或遥测信息
