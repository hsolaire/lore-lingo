# LensLingo · Tauri 迁移说明

记录本次将 Vue 3 + Vite 纯前端项目改造为 Tauri 2 桌面客户端的所有改动点及后续操作步骤。

---

## 已改动的文件

### `vite.config.ts`
- 新增 `clearScreen: false`：保留终端中 Rust/Cargo 日志
- 新增 `server.port: 5173` + `strictPort: true`：Tauri dev 需要固定端口，被占用时直接报错
- 新增 `server.watch.ignored: ['**/src-tauri/**']`：避免 Vite 热更新监听 Rust 文件
- 新增 `build.target`：指定 Tauri WebView 兼容的最低目标（`es2021 / chrome105 / safari13`）
- 新增 `build.minify` 和 `build.sourcemap`：调试模式（`TAURI_DEBUG=1`）下关闭压缩、开启 sourcemap
- 新增 `envPrefix: ['VITE_', 'TAURI_']`：让 `TAURI_*` 环境变量也能在前端读取

### `package.json`
- 新增依赖 `@tauri-apps/api@^2`：前端调用 Tauri 原生 API 的 JS 库
- 新增开发依赖 `@tauri-apps/cli@^2`：`cargo tauri` 命令的 npm 封装
- 新增脚本 `tauri:dev` / `tauri:build`

### `tsconfig.json`
- `types` 数组新增 `"@tauri-apps/api"`，补全 Tauri API 的 TypeScript 类型

### `src/composables/useTranslator.ts`
- `togglePin` 改为 `async`
- 调用 `getCurrentWindow().setAlwaysOnTop(pinned.value)` 真实控制原生窗口层级，不再只是 UI 状态

### `src/components/main-panel/TitleBar.vue`
- 引入 `getCurrentWindow` 并绑定红点 → `win.close()`、黄点 → `win.minimize()`
- `.traffic` 容器加 `-webkit-app-region: no-drag`，防止点击灯触发拖拽
- 绿点保持视觉占位，暂不绑定（macOS 全屏逻辑较复杂）

### `src/styles/global.css`
- 移除 `body` 上的壁纸渐变背景、`display: grid / place-items: center / padding`
- 改为 `background: transparent`，Tauri 透明窗口直接透出系统桌面
- 保留壁纸变量定义（`--wallpaper-*`），用 `@media` + `.preview-mode` 包裹，供浏览器调试时手动挂 class 还原预览效果

---

## 新增的文件

```
src-tauri/
├── Cargo.toml              # Rust 项目 + 依赖（screenshots、base64）
├── build.rs                # Tauri 构建钩子（必须存在）
├── tauri.conf.json         # 窗口配置：无边框、透明、420×640
├── capabilities/
│   └── default.json        # 权限声明：置顶、关闭、最小化、shell
└── src/
    ├── main.rs             # 二进制入口（调 lib::run）
    └── lib.rs              # 核心逻辑：capture_region、list_screens 命令
```

### `tauri.conf.json` 关键配置
| 字段 | 值 | 原因 |
|------|----|------|
| `decorations` | `false` | 去掉系统原生标题栏，使用自定义 traffic 灯 |
| `transparent` | `true` | 窗口背景透出桌面，配合 glassmorphism UI |
| `shadow` | `false` | 透明窗口开阴影会在 macOS 上留白边 |
| `alwaysOnTop` | `false` | 初始不置顶，由 pin 按钮动态控制 |

### `src-tauri/src/lib.rs` 暴露的命令
| 命令 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `capture_region` | `x, y, w, h: i32/u32` | `Result<String>` PNG base64 | 截取指定区域 |
| `list_screens` | 无 | `Vec<JSON>` | 列出所有屏幕信息 |

前端调用示例：
```ts
import { invoke } from '@tauri-apps/api/core'

// 截图
const png = await invoke<string>('capture_region', { x: 700, y: 46, w: 520, h: 118 })

// 列出屏幕
const screens = await invoke('list_screens')
```

---

## 接下来你需要手动做的事

### 第一步：安装 Rust（如果没装）
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 第二步：安装前端依赖
```bash
pnpm install
```

### 第三步：准备图标

`src-tauri/icons/` 目录目前为空，Tauri 构建时必须有图标文件。最快的方法是用 Tauri CLI 自动生成：

```bash
# 准备一张 1024×1024 的 PNG 源图，放到项目根目录，命名为 app-icon.png
pnpm tauri icon app-icon.png
```

这会自动生成 `icons/` 下所需的所有尺寸（`.png`、`.icns`、`.ico`）。

### 第四步：启动开发模式
```bash
pnpm tauri:dev
# 等价于：cargo tauri dev
```

首次运行会编译 Rust 依赖，大约需要 2–5 分钟，后续热更新秒级响应。

### 第五步：打包发布
```bash
pnpm tauri:build
```

产物在 `src-tauri/target/release/bundle/`：
- macOS → `LensLingo.app` + `LensLingo.dmg`
- Windows → `LensLingo_x64-setup.exe` + `.msi`

---

## 尚未实现（后续迭代）

| 功能 | 现状 | 建议方向 |
|------|------|----------|
| 框选区域 | 模拟 1.9s 延迟 | 创建全屏透明 Tauri 覆盖窗口，监听 mousedown/mousemove/mouseup 取坐标 |
| 捕获应用窗口列表 | 硬编码 VLC | 用 `tauri-plugin-fs` 或系统 API 枚举进程窗口 |
| OCR 识别 | 无 | Rust 端接入 `tesseract-rs` 或调用系统 OCR（macOS Vision Framework） |
| 翻译 API | 模拟静态文本 | `useTranslator` 中 `invoke('translate', { text, from, to })` |
| 系统托盘 | 配置了 trayIcon | 需在 `lib.rs` 中注册 `SystemTray` 事件处理 |
| macOS 屏幕录制权限 | `infoPlist` 已声明 | 首次截图时 macOS 会弹权限弹窗，无需额外处理 |
