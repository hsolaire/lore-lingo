# 发布清单

## 目录

- [准备](#准备)
- [质量门槛](#质量门槛)
- [候选产物](#候选产物)
- [发布](#发布)
- [回滚与记录](#回滚与记录)

## 准备

- [ ] 当前提交来自 `main`，工作区干净，远端状态已确认。
- [ ] `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 使用同一个非 `0.0.0` SemVer。
- [ ] 标签格式为 `vMAJOR.MINOR.PATCH`，且与文件版本一致。
- [ ] `pnpm-lock.yaml` 是唯一 JavaScript 锁文件，依赖使用 frozen 模式安装。
- [ ] Tauri identifier 稳定，图标、产品名、最低系统版本和版权信息正确。
- [ ] 生产 CSP 已配置，不再为 `null`。
- [ ] macOS 用途说明和 entitlements 最小化；Windows WebView2 模式明确。
- [ ] 不存在证书、私钥、密码、token、`.env` 或安装器的待提交文件。
- [ ] OpenSpec 任务和发布说明与实际能力一致，不把模拟或仅发现能力描述成完整捕获能力。

## 质量门槛

在两个原生平台分别执行适用检查：

```bash
pnpm install --frozen-lockfile
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
git diff --check
```

- [ ] Windows 原生 CI 编译所有 Windows target-gated 模块。
- [ ] macOS 原生 CI 编译 arm64 和 x86_64，并检查 Universal 2 架构。
- [ ] 全新 SQLite 初始化和从上一正式版本升级迁移通过。
- [ ] 无严重或高危依赖公告；无法处理的公告有风险接受记录。
- [ ] 权限、窗口发现、区域/窗口捕获、多显示器、缩放和悬浮字幕矩阵已执行。

## 候选产物

每个平台记录：

- Git commit、标签、构建 job URL、宿主 OS 和架构。
- Tauri、Rust、Node.js、pnpm 版本。
- 产物文件名、大小和 SHA-256。
- 签名主体、证书有效期、时间戳和验证命令结果。
- macOS 公证 submission ID 与 stapling 结果；不得记录凭据。
- Windows 安装器格式和 WebView2 模式。

候选版本至少验证：

- [ ] 安装、首次启动、退出和再次启动。
- [ ] macOS 权限授权/拒绝/撤销；Windows 受保护或高权限来源失败。
- [ ] 主副显示器、负坐标、Retina 或 100/150/200% 缩放。
- [ ] 从上一正式版本升级，设置和数据库数据保留。
- [ ] 卸载行为和重新安装。

## 发布

执行标签、推送、创建 Release 或上传产物前再次获得用户明确授权。

- [ ] 标签指向已验证的 `main` 提交。
- [ ] 上传的文件就是完成签名和烟雾测试的文件，没有发布后重新打包。
- [ ] Release 同时提供 SHA-256 校验和。
- [ ] 发布说明包含用户可见变化、最低 OS、架构、迁移、已知限制和恢复方式。
- [ ] Windows 与 macOS 产物名称能明确区分平台、架构、版本和安装器格式。

## 回滚与记录

- 安装器发布错误：撤下有问题的附件，保留审计记录，发布修正版 PATCH；不要静默替换同名文件。
- 签名或公证失效：停止分发并吊销/轮换受影响凭据；不得在仓库提交新凭据。
- 数据迁移失败：停止升级推广，按已验证恢复步骤处理；不得修改已发布迁移文件。
- 捕获或权限严重回归：记录受影响平台和版本，准备可验证的 hotfix，并同步 `main` 与 `develop`。

最终报告必须分别列出“已验证平台”和“未验证平台”，以及仍未完成的发布门槛。
