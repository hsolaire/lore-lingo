# macOS 打包、签名与公证

## 目录

- [目标与前置条件](#目标与前置条件)
- [一次性配置](#一次性配置)
- [无签名验证](#无签名验证)
- [正式构建](#正式构建)
- [签名与公证验证](#签名与公证验证)
- [常见故障](#常见故障)

## 目标与前置条件

- 宿主机：macOS 13 或更高版本。
- 发布架构：`aarch64-apple-darwin` 和 `x86_64-apple-darwin`，最终为 Universal 2。
- Xcode Command Line Tools、Node.js、pnpm、Rust/Cargo 和 Tauri CLI 可用。
- 正式直发包使用 Apple Developer Program 的 `Developer ID Application` 身份。
- 屏幕录制用途说明保留在 `src-tauri/Info.plist`；发布前实测首次授权、拒绝、撤销和重启流程。

macOS 只能构建 macOS 产物。Intel 兼容性至少要通过目标编译和原生 CI；不能只在 Apple Silicon 上运行 arm64 包后声称 Universal 2 已验证。

## 一次性配置

### 应用配置

发布前确认 `src-tauri/tauri.conf.json`：

- `identifier` 是稳定反向域名，且不要以 `.app` 结尾。
- `bundle.macOS.minimumSystemVersion` 与项目支持范围一致，当前要求为 `13.0`。
- `bundle.macOS.infoPlist` 指向 `Info.plist`。
- 仅在确有原生授权需求时增加 entitlements；不要复制无关权限。
- 生产 CSP 不得为 `null`。

### Rust 目标

使用仓库锁定的 Rust 工具链安装两个目标：

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

### 签名身份

本地授权发布机可从钥匙串确认身份：

```bash
security find-identity -v -p codesigning
```

通过 `APPLE_SIGNING_IDENTITY` 指定 `Developer ID Application: ...`。CI 通常把 base64 编码的 PKCS#12 证书和密码保存在受保护 secrets 中，并在临时钥匙串导入；不得把证书或密码写入仓库。

### 公证身份

二选一：

1. App Store Connect API key：优先用于 CI，使用 issuer、key ID 和临时 `.p8` 文件。
2. Apple ID：使用 `APPLE_ID`、应用专用密码和 `APPLE_TEAM_ID`，不得使用 Apple ID 主密码。

采用的环境变量名称必须与当前 Tauri 2 文档和 CI action 版本核对。预检只验证本仓库约定的名称是否存在，不证明 Apple 凭据有效。

## 无签名验证

先运行项目质量门槛，再执行两个目标的编译或无签名 bundle 构建。开发烟雾包可执行：

```bash
pnpm tauri build --debug --target aarch64-apple-darwin
```

这类包仅用于本机验证，不得作为正式分发产物。记录目标架构和实际运行机器。

## 正式构建

1. 在干净的 `main` 提交和语义化版本标签上运行 `release` 预检。
2. 确认两个 Rust 目标、Developer ID 身份和公证凭据可用。
3. 使用 Tauri 2 当前支持的 Universal 目标构建：

   ```bash
   pnpm tauri build --target universal-apple-darwin
   ```

4. 保留 Tauri 输出的 `.app` 和 `.dmg` 路径，禁止手工拼测出的 target 路径。
5. 检查主程序同时包含两个架构：

   ```bash
   lipo -archs "LoreLingo.app/Contents/MacOS/LoreLingo"
   ```

预期同时出现 `x86_64` 和 `arm64`。如果 Tauri 未自动完成公证，使用 `xcrun notarytool submit ... --wait` 提交已签名产物，并在成功后执行 `xcrun stapler staple`。不要把公证失败降级为发布警告。

## 签名与公证验证

对最终 `.app` 和 `.dmg` 执行：

```bash
codesign --verify --deep --strict --verbose=2 LoreLingo.app
codesign -dv --verbose=4 LoreLingo.app
spctl --assess --type execute --verbose=4 LoreLingo.app
xcrun stapler validate LoreLingo.app
xcrun stapler validate LoreLingo.dmg
```

然后在至少一台没有开发环境状态污染的测试机上：

1. 挂载 DMG 并拖入 `/Applications`。
2. 首次启动，确认 Gatekeeper 不需要右键绕过。
3. 验证录屏权限未提前弹出，只在用户打开来源或开始捕获时请求。
4. 验证授权、拒绝、撤销、重启和升级安装。
5. 卸载应用，确认不会误删用户无关数据。

为最终 DMG 生成 SHA-256，并在 Release 中公布。

## 常见故障

| 现象 | 处理 |
| --- | --- |
| `no identity found` | 检查证书是否包含私钥、是否在当前钥匙串、身份字符串是否精确 |
| 公证返回 `Invalid` | 下载 notary 日志，修复签名、hardened runtime、嵌套二进制或权限问题后重建 |
| `spctl` 拒绝但 `codesign` 通过 | 检查 Developer ID、时间戳、公证和 stapling；`codesign` 通过不等于可分发 |
| Intel 目标链接失败 | 检查所有 Rust/C 原生依赖是否包含 x86_64，不得删除 Intel 目标掩盖问题 |
| 录屏权限描述缺失 | 检查 bundle 内最终 `Info.plist`，不要只看源文件 |
| bundle ID 警告 | 改用不以 `.app` 结尾的稳定标识，并评估权限身份迁移影响 |
