# 🛠️ 开发指南

本指南帮助开发者在 Windows、macOS 和 Linux 上进行跨平台开发。

---

## 📋 环境设置

### Windows 开发环境

1. **安装必要工具**
   ```bash
   # 使用 scoop 或 chocolatey
   scoop install rustup nodejs pnpm
   # 或
   choco install rustup.install nodejs pnpm
   ```

2. **配置 Git 换行符**
   ```bash
   git config --global core.autocrlf true
   ```

3. **启用 Rust 工具链**
   ```bash
   rustup default stable
   rustup component add rust-analyzer
   ```

### macOS 开发环境

1. **安装必要工具**
   ```bash
   # 使用 Homebrew
   brew install rustup node pnpm
   ```

2. **配置 Git**
   ```bash
   git config --global core.autocrlf input
   ```

3. **设置 Rust**
   ```bash
   rustup-init
   rustup default stable
   ```

### Linux 开发环境

```bash
# Ubuntu/Debian
sudo apt-get install build-essential curl npm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
npm install -g pnpm
```

---

## 🔧 项目配置

### 跨平台换行符处理

项目已配置 `.gitattributes` 和 `.editorconfig`：

- **自动换行符转换**：所有源代码文件自动使用 LF 换行
- **编辑器配置**：支持所有主流编辑器（VSCode、JetBrains、Sublime 等）
- **Git 配置**：通过 `core.autocrlf` 自动处理平台差异

**不需要手动配置**，Git 会自动在提交时转换为 LF，签出时转换为平台默认值。

### 推荐 IDE 配置

#### VS Code

安装扩展：
- **EditorConfig for VS Code**
- **ESLint**
- **Prettier**
- **Rust Analyzer**
- **Vue - Official**

`.vscode/settings.json`：
```json
{
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": "explicit"
  },
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

#### JetBrains (WebStorm / IntelliJ IDEA)

1. 设置 > 编辑器 > 代码风格
   - 行分隔符：`LF (\n)`
   - 右边距：120 字符

2. 设置 > 工具 > 文件和IDE主题
   - 启用 EditorConfig 支持

3. 设置 > 语言和框架 > Rust
   - 启用 Rust Analyzer

---

## 🚀 开发工作流

### 快速开始

```bash
# 克隆仓库
git clone https://github.com/blake511/lore-lingo.git
cd lore-lingo

# 安装依赖
pnpm install

# 启动开发服务器
pnpm tauri:dev
```

### 常见命令

```bash
# 前端开发（Vite 热重载）
pnpm dev

# Rust 后端编译
cargo build -p lenslingo

# 完整开发构建
pnpm tauri:dev

# 生产构建
pnpm tauri:build

# 代码检查
pnpm lint

# 类型检查
pnpm type-check

# 运行测试
pnpm test
```

### Git 工作流

```bash
# 创建新分支
git checkout -b feature/your-feature-name

# 提交更改（自动格式化）
git add .
git commit -m "feat: add new feature"

# 推送到远程
git push origin feature/your-feature-name

# 创建 Pull Request
# 在 GitHub 上创建 PR，等待 CI 检查通过
```

**提交信息规范**（Conventional Commits）：
```
feat:      新功能
fix:       bug 修复
refactor:  代码重构
test:      测试相关
docs:      文档更新
chore:     配置、依赖更新
style:     代码格式（不影响功能）
perf:      性能优化
```

---

## 🐛 调试指南

### 前端调试

1. **Vite DevTools**
   ```bash
   pnpm dev
   # 访问 http://localhost:5173
   # 打开浏览器开发者工具 (F12)
   ```

2. **Vue DevTools**
   - 安装浏览器扩展（Chrome / Firefox）
   - 在浏览器开发者工具中查看组件树

3. **断点调试**
   - VS Code：安装 "Debugger for Chrome" 扩展
   - 设置 `.vscode/launch.json`

### Rust 后端调试

1. **日志输出**
   ```rust
   // 在 lib.rs 中使用 println! 或日志库
   println!("Debug: {:?}", variable);
   ```

2. **Rust Analyzer**
   - 在 IDE 中使用 "Debug" 功能
   - 支持在 Rust 代码中设置断点

3. **Tauri 开发者工具**
   ```bash
   # 开发模式自动打开
   pnpm tauri:dev
   # 按 F12 打开开发者工具
   ```

### 常见问题排查

| 问题 | 原因 | 解决方案 |
|------|------|--------|
| 行尾显示 CRLF | 换行符配置不一致 | 运行 `git add --renormalize -A` |
| Rust 编译失败 | 缺少依赖或版本不匹配 | 运行 `cargo update` |
| 热重载不工作 | Vite 端口被占用 | 检查 `src-tauri/tauri.conf.json` 中的 `devUrl` |
| Node 模块丢失 | 跨平台路径问题 | 运行 `pnpm install --force` |

---

## 📦 依赖管理

### 前端依赖

- **Vue 3**：UI 框架
- **TypeScript**：类型检查
- **Vite**：构建工具
- **@tauri-apps/api**：Tauri 前端 API

升级依赖：
```bash
pnpm up
pnpm update @tauri-apps/api
```

### Rust 依赖

编辑 `src-tauri/Cargo.toml`：
```bash
# 添加依赖
cargo add package-name

# 更新所有依赖
cargo update

# 检查安全漏洞
cargo audit
```

---

## 🔍 代码质量

### 类型检查

```bash
# 检查 TypeScript 类型
pnpm type-check
```

### 代码格式化

```bash
# 格式化所有代码
pnpm format

# 只检查格式（不修改）
pnpm format:check
```

### Linting

```bash
# ESLint for JavaScript/TypeScript
pnpm lint

# Clippy for Rust
cargo clippy
```

### 单元测试

```bash
# 运行前端测试
pnpm test

# 运行 Rust 测试
cargo test -p lenslingo
```

---

## 📱 平台特定注意事项

### Windows

- **CRLF 处理**：已自动配置，勿手动修改
- **路径分隔符**：使用正斜杠 (`/`)，Rust 和 Node 都支持
- **权限**：某些功能需要管理员权限（后续实现）

### macOS

- **M1/M2 芯片**：Rust 工具链会自动选择正确的架构
- **代码签名**：生产构建时需要配置证书（参见 `src-tauri/tauri.conf.json`)
- **权限**：需要录屏权限（在系统偏好设置中授予）

### Linux

- **X11 vs Wayland**：目前主要支持 X11
- **GTK 主题**：UI 会遵循系统主题色
- **权限**：某些截屏功能可能需要 `root` 权限

---

## 🚢 发布流程

### 预发布检查

```bash
# 1. 运行完整的测试和检查
pnpm test
pnpm lint
pnpm type-check
cargo test -p lenslingo
cargo clippy

# 2. 更新版本
# 编辑 package.json 和 src-tauri/Cargo.toml 中的 version

# 3. 更新 CHANGELOG
# 记录新功能和修复

# 4. 创建 git tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

### 构建发布版本

```bash
# 构建所有平台
pnpm tauri:build

# 输出位置：
# - Windows: src-tauri/target/release/bundle/msi/
# - macOS: src-tauri/target/release/bundle/macos/
# - Linux: src-tauri/target/release/bundle/deb/
```

### 发布到 GitHub

```bash
# 创建 Release Notes
# GitHub 会自动从 tag 生成 Release

# 上传构建产物
# 将 .exe / .dmg / .AppImage 上传到 Release
```

---

## 💡 最佳实践

### 代码风格

- ✅ 使用 TypeScript，避免 `any` 类型
- ✅ 组件名用 PascalCase，文件名用 kebab-case
- ✅ Composables 用 `use` 前缀
- ✅ 两空格缩进（JavaScript/TypeScript/Vue）
- ✅ 四空格缩进（Rust）

### 性能优化

- ✅ 使用 Vue 3 Composition API 的 `ref` 和 `reactive`
- ✅ 避免不必要的渲染，使用 `v-show` 或 `v-if`
- ✅ Rust 中使用 `async/await` 处理异步操作
- ✅ 定期审查和优化捕获延迟

### 安全性

- ✅ 不在代码中硬编码 API 密钥（使用环境变量）
- ✅ 验证用户输入和外部数据
- ✅ 避免评估用户输入的代码
- ✅ 定期检查依赖的安全漏洞（`cargo audit`）

---

## 📞 获取帮助

- **问题追踪**：[GitHub Issues](https://github.com/blake511/lore-lingo/issues)
- **讨论区**：[GitHub Discussions](https://github.com/blake511/lore-lingo/discussions)
- **文档**：查看本项目的 [README.md](./README.md)

---

**Happy Coding! 🎉**
