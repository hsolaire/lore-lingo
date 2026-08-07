---
name: lore-lingo-release-packaging
description: 在准备、构建、签名、公证、打包、验证或发布 LoreLingo 的 macOS 与 Windows 桌面产物时使用，适用于 DMG、app、NSIS、MSI、Universal 2、Developer ID、Apple 公证、Authenticode、WebView2、校验和与发布标签。
---

# LoreLingo 发布打包

将发布视为原生平台流水线，而不是一条跨平台本地命令。macOS 产物只在 macOS 构建，Windows 产物只在 Windows 构建；正式产物必须签名并通过安装验证。

**必须同时使用：** `lore-lingo-project-standards`，特别是其中的发布、安全、测试和 Git 规则。

## 入口

1. 先从仓库根目录运行预检：

   ```bash
   node .agents/skills/lore-lingo-release-packaging/scripts/release-preflight.mjs --mode verify
   ```

2. 根据目标平台读取对应参考：
   - macOS、Universal 2、Developer ID、公证、DMG：读取 `references/macos.md`。
   - Windows、NSIS/MSI、WebView2、Authenticode：读取 `references/windows.md`。
   - 版本、Git、测试、校验和、发布与回滚：始终读取 `references/release-checklist.md`。
3. 修复所有 `FAIL`。`WARN` 必须在发布记录中解释；不得静默忽略。
4. 在原生宿主机执行无签名验证构建，再执行正式签名构建。
5. 对最终安装器做签名、安装、首次启动、升级和卸载验证，而不只检查编译成功。

## 模式

| 模式 | 用途 | 凭据 |
| --- | --- | --- |
| `verify` | 本地开发或 PR 的无签名构建 | 不要求，也不得索取或伪造 |
| `release` | 受保护 CI 或授权发布机上的正式产物 | 只检查变量是否存在，不输出值 |

正式预检示例：

```bash
node .agents/skills/lore-lingo-release-packaging/scripts/release-preflight.mjs \
  --mode release --platform macos
```

支持 `--platform auto|macos|windows|all` 和 `--allow-dirty`。`all` 仅用于 `verify` 配置审计；`release` 模式必须指定单个平台，且不得使用 `--allow-dirty`。

## 强制边界

- 不提交证书、私钥、描述文件、API key、应用专用密码、`.env` 或生成的安装器。
- 不打印、解码、回显或写入签名凭据；日志只报告变量是否存在。
- 不把 `TAURI_SIGNING_PRIVATE_KEY` 当作 Windows Authenticode 证书；它用于 Tauri 更新包签名，不签署安装器。
- 不在不受信任 PR 中启用签名或公证 secrets。
- 不声称 macOS 主机可生成受支持的 Windows 安装器，反之亦然。
- 不声称未签名包、ad-hoc 签名包或仅通过编译的包可正式发布。
- 不自动创建标签、推送、上传 Release 或覆盖既有产物；这些动作需要用户明确授权。
- 不因打包失败而关闭 Gatekeeper、SmartScreen、防病毒或系统安全策略。

## 完成标准

只有同时满足以下条件才能报告正式发布完成：

- 三处版本一致，发布标签指向 `main` 上经过验证的提交。
- 前端、Rust、数据库迁移和平台原生构建全部通过。
- macOS 产物完成 Developer ID 签名、公证与 stapling；Windows 产物完成 Authenticode 签名和时间戳。
- 最终安装器通过签名验证、校验和验证及安装生命周期烟雾测试。
- 发布说明记录最低系统版本、架构、WebView2 策略、迁移、已知限制和回滚方式。

任何平台没有原生验证证据时，明确写为“未验证”，不得用另一平台结果推断。
