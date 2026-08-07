# Windows 打包与签名

## 目录

- [目标与前置条件](#目标与前置条件)
- [一次性配置](#一次性配置)
- [无签名验证](#无签名验证)
- [正式构建与签名](#正式构建与签名)
- [安装验证](#安装验证)
- [常见故障](#常见故障)

## 目标与前置条件

- 宿主机：原生 Windows 10 22H2 或 Windows 11。
- 目标：`x86_64-pc-windows-msvc`。
- Visual Studio Build Tools 包含 Desktop development with C++ 和 Windows SDK。
- Node.js、pnpm、Rust/Cargo、Tauri CLI 与 WebView2 Runtime 可用。
- 正式安装器使用受信任 CA 颁发的代码签名证书或 Azure Trusted Signing。

不能在 macOS 本地构建后声称 Windows 安装器可用。Windows 平台适配器、NSIS/MSI、WebView2 和签名必须在 Windows runner 或实机验证。

## 一次性配置

### Rust 与安装器工具

```powershell
rustup target add x86_64-pc-windows-msvc
pnpm install --frozen-lockfile
```

构建 MSI 时确认 WiX/Tauri 当前版本所需工具已经安装；构建 NSIS 时使用 Tauri 管理的兼容工具链，不要从未知镜像下载脚本或二进制。

### WebView2 策略

发布前在 `src-tauri/tauri.conf.json` 明确 Windows `webviewInstallMode`，并在发布说明中记录：

- `downloadBootstrapper`：安装时联网下载，包较小但依赖网络。
- `embedBootstrapper`：内置引导器，仍可能联网获取 Runtime。
- `offlineInstaller`：包更大，适合离线环境。
- `skip`：仅在受控环境已保证 Runtime 时使用。

LoreLingo 支持普通用户安装时，不得默认假设 WebView2 已存在。策略改变必须重新验证全新机器和离线行为。

### Authenticode

证书私钥保存在 Windows 证书存储、硬件令牌、受保护 CI secret 或 Azure Trusted Signing 中。Tauri 配置可以声明证书 thumbprint、SHA-256 digest 和 RFC 3161 时间戳 URL；具体字段必须与仓库锁定的 Tauri 2 schema 核对。

禁止：

- 把 `.pfx`、密码或导出的私钥提交到仓库。
- 在日志打印 secret 或完整证书内容。
- 用自签名证书冒充正式分发签名。
- 把 Tauri updater 的私钥当作 Authenticode 证书。

## 无签名验证

在 PR 或无凭据环境执行：

```powershell
node .agents/skills/lore-lingo-release-packaging/scripts/release-preflight.mjs --mode verify --platform windows
pnpm install --frozen-lockfile
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
pnpm tauri build --debug --target x86_64-pc-windows-msvc
```

无签名产物用于编译和安装器结构验证，不发布给终端用户。

## 正式构建与签名

1. 在干净的 `main` 和语义化版本标签上运行：

   ```powershell
   node .agents/skills/lore-lingo-release-packaging/scripts/release-preflight.mjs --mode release --platform windows
   ```

2. 在受保护 job 中导入证书或建立 Azure Trusted Signing 会话。
3. 使用明确目标构建，不依赖宿主默认值：

   ```powershell
   pnpm tauri build --target x86_64-pc-windows-msvc
   ```

4. 对 Tauri 输出的 `.exe`、`.msi` 或安装器内关键可执行文件核验 Authenticode：

   ```powershell
   Get-AuthenticodeSignature .\path\to\LoreLingo.exe | Format-List
   ```

5. 状态必须为 `Valid`，证书主题符合发布主体，签名使用 SHA-256，并具有可信时间戳。签名失败时不得发布未签名替代品。

如果同时产出 NSIS 和 MSI，分别验证；一个格式通过不能代表另一个格式可安装。

## 安装验证

至少覆盖 Windows 10 22H2 和 Windows 11：

1. 在无开发工具的干净 x86_64 环境安装。
2. 验证标准用户安装、首次启动、开始菜单和卸载入口。
3. 验证 WebView2 已安装、在线失败和离线策略。
4. 验证窗口来源发现、100/150/200% 缩放、多显示器和受保护/管理员窗口错误。
5. 从上一正式版本升级并检查 SQLite 迁移与用户设置。
6. 卸载并确认不会删除用户无关文件。
7. 生成 SHA-256：

   ```powershell
   Get-FileHash .\path\to\installer.exe -Algorithm SHA256
   ```

## 常见故障

| 现象 | 处理 |
| --- | --- |
| `link.exe` 或 Windows SDK 缺失 | 修复 Visual Studio Build Tools 工作负载和 MSVC 环境 |
| Authenticode 状态不是 `Valid` | 检查证书链、用途、有效期、SHA-256 和时间戳服务 |
| CI 找不到证书 | 检查导入用户/机器证书存储是否与构建进程一致，不输出密码排查 |
| 安装后白屏 | 检查 WebView2 策略、Runtime 版本、CSP 和前端资源路径 |
| SmartScreen 提示 | 先确认签名有效；新证书信誉需要累积，不能通过关闭 SmartScreen 作为修复 |
| MSI 通过但 NSIS 失败 | 分别检查安装作用域、升级代码、文件占用和卸载逻辑 |
| 管理员窗口无法捕获 | 返回结构化不可用错误并记录限制，不要求应用整体提权 |
