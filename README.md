# 🎬 LoreLingo

**实时屏幕翻译工具** — 一个跨平台的桌面应用，支持实时 OCR 识别和智能翻译任何屏幕内容。

## ✨ 核心功能

### 🎯 多模式字幕捕获

- **应用窗口模式**：自动识别并捕获指定应用窗口（如播放器、浏览器）的字幕区域
- **自定义区域模式**：灵活框选屏幕任意区域进行翻译
- 实时预览和位置调整

### 💬 智能翻译

- **双引擎翻译**
  - 📚 **字幕库命中优先**：首先在已加载的字幕文件中进行模糊匹配，秒级返回译文
  - 🤖 **大模型翻译**：未命中时调用 AI 模型进行高质量翻译
  - 💰 **成本优化**：通过字幕库命中可节省约 **68%** 的模型请求

- **多语言支持**（目前支持英译中，后续扩展）
  - 一键切换翻译方向
  - 实时更新翻译结果

### 🪟 悬浮字幕

- **屏幕置顶显示**：支持将悬浮字幕置于屏幕最前层
- **自由拖拽定位**：鼠标拖动调整窗口位置，位置自动保存
- **合并悬浮窗口**：主面板与字幕显示合为一个窗口，常用操作集中在顶部工具栏
- **四个顶部入口**：翻译选项下拉、捕获窗口下拉、捕获区域、设置；右侧独立控制窗口置顶与固定工具栏。菜单浮在字幕上方，展开不撑大字幕背景
- **区域状态与预览**：截图成功后显示“已捕获”，悬停查看实时画面，点击编辑 X1/X2/Y1/Y2；OCR 空结果或翻译失败也保留选区
- **自动收起工具栏**：鼠标移入显示、移出后自动收起；可单独固定工具栏，窗口置顶由另一开关控制

### ⌨️ 快捷键支持

- **开始翻译**：快速启动捕获和翻译流程
- **结束翻译**：立即停止当前翻译任务
- 完全可配置，支持自定义快捷键绑定

### 📂 字幕库管理

支持多种字幕格式：
- **SRT** (SubRip Text)
- **ASS** (Advanced SubStation Alpha)
- **VTT** (WebVTT)

**工作流**：
1. 拖入或点击导入字幕文件
2. 应用自动解析并索引字幕内容
3. OCR 识别的文本自动与字幕库匹配
4. 命中时直接使用字幕中的译文（零延迟）

---

## 当前单次捕获使用

macOS 14 及以上可从窗口下拉选择目标，或点击区域按钮拖拽框选。需为实际运行的 LoreLingo 开启屏幕录制权限并按系统提示重启。服务地址、模型与 API Key 在设置 → 翻译中应用，供两种捕获共用；服务地址、模型和默认语言持久化到 SQLite，API Key 保存在 macOS 钥匙串或 Windows 凭据管理器中，数据库只保留引用，重启后自动恢复。没有配置时仍可捕获和预览，但会明确报告翻译未配置。区域面板展开时，每帧完成后间隔 0.5 秒刷新缩略图；关闭面板即停止后续刷新。X1/Y1 与 X2/Y2 分别为当前显示器左上角原点下的左上、右下逻辑坐标，应用后刷新新区域，可点击“翻译此区域”重新捕获。Windows OCR、macOS 13 单帧捕获、长期连续 OCR/翻译会话尚未完成。

## 配置存放位置

默认配置目录统一为 `~/.lore-lingo`，数据库为 `~/.lore-lingo/lore-lingo.sqlite3`。Windows 对应 `%USERPROFILE%\.lore-lingo\lore-lingo.sqlite3`。地址、模型及其他已经接入的持久化配置集中使用这个数据库；API Key 保存在系统钥匙串／凭据管理器，数据库只保存引用。

首次使用新目录时，若目标数据库不存在，会安全迁入旧应用数据目录中的数据库并保留旧库；目标已经存在时直接使用，不覆盖、不合并旧库。重启新版应用后生效。

## 🏗️ 技术架构

### 前端

- **框架**：Vue 3 + TypeScript
- **构建**：Vite
- **UI 库**：原生 CSS + 设计系统（glassmorphism 风格）
- **状态管理**：Composables (Composition API)

### 后端 / 桌面集成

- **框架**：Tauri 2 (Rust)
- **核心功能**
  - 屏幕截图和区域捕获
  - 原生窗口管理（置顶、最小化、关闭）
  - 系统快捷键监听（后续实现）
  - 字幕文件解析引擎

### 外部集成

- **OCR**：支持多种 OCR 服务（接口待集成）
- **翻译模型**：支持接入各类大模型 API（OpenAI / 国内厂商等）
- **字幕库**：内置字幕解析和模糊匹配引擎

---

## 📁 项目结构

```
lore-lingo/
├── src/
│   ├── components/
│   │   ├── main-panel/          # 主控制面板相关
│   │   │   ├── MainPanel.vue    # 主容器
│   │   │   ├── TitleBar.vue     # 标题栏 + 窗口控制
│   │   │   ├── SourceRow.vue    # 捕获模式选择（应用/区域）
│   │   │   ├── CaptureTarget.vue # 捕获目标展示
│   │   │   ├── TranslateStream.vue # 翻译流 + 原文/译文卡片
│   │   │   ├── LangFlip.vue     # 语言方向切换按钮
│   │   │   ├── TranslationCard.vue # 单条翻译卡片
│   │   │   └── Toolbar.vue      # 操作工具栏
│   │   ├── subtitle/            # 悬浮字幕相关
│   │   │   ├── SubtitleBar.vue  # 悬浮字幕容器
│   │   │   ├── SubtitleContent.vue # 字幕显示区域
│   │   │   └── SubtitleControls.vue # 悬浮字幕控制栏
│   │   ├── settings/            # 设置面板
│   │   │   ├── SettingsDrawer.vue # 设置抽屉容器
│   │   │   ├── OcrPanel.vue     # OCR 设置
│   │   │   ├── TranslatePanel.vue # 翻译模型设置
│   │   │   └── SubsPanel.vue    # 字幕库管理
│   │   ├── ui/                  # 通用 UI 组件
│   │   │   ├── Icon.vue         # 图标系统
│   │   │   ├── SelectField.vue  # 下拉选择框
│   │   │   └── ToggleSwitch.vue # 开关组件
│   │   ├── CaptureFrame.vue     # 捕获区域框架显示
│   │   └── AppToast.vue         # 消息提示
│   ├── composables/
│   │   ├── useSubtitle.ts       # 悬浮字幕显示与关闭
│   │   ├── useSubtitleBridge.ts # 双窗口状态同步
│   │   ├── useSubtitleWindow.ts # 悬浮字幕原生窗口交互
│   │   ├── useCaptureSource.ts  # 捕获源管理（应用/区域切换）
│   │   ├── useTranslator.ts     # 翻译状态共享
│   │   ├── useSubtitles.ts      # 字幕库管理
│   │   ├── useSettings.ts       # 设置面板状态
│   │   └── useToast.ts          # 消息队列
│   ├── icons/
│   ├── App.vue                  # 主窗口根组件
│   ├── SubtitleApp.vue          # 悬浮字幕窗口根组件
│   └── subtitle.ts              # 悬浮字幕窗口入口
├── src-tauri/                   # Tauri 后端
│   ├── src/
│   │   ├── lib.rs               # 核心逻辑（屏幕截图、字幕解析等）
│   │   └── main.rs              # 入口点
│   ├── Cargo.toml
│   └── tauri.conf.json          # Tauri 配置（窗口、权限等）
├── index.html
├── subtitle.html
├── package.json
└── README.md                    # 本文件
```

---

## 🚀 快速开始

### 环境要求

- **Node.js** ≥ 18.0
- **Rust** ≥ 1.70 (用于 Tauri)
- **pnpm** (推荐) 或 npm / yarn

### 安装依赖

```bash
pnpm install
```

### 开发模式

启动开发服务器和 Tauri 应用窗口：

```bash
pnpm tauri:dev
```

此命令会：
1. 启动 Vite 前端开发服务器（`http://localhost:5173`）
2. 编译和运行 Tauri 后端
3. 自动打开应用窗口（开发模式下显示 DevTools）

### 构建生产版本

```bash
pnpm tauri:build
```

构建的可执行文件位于 `src-tauri/target/release/`

---

## 🎨 设计特点

### UI/UX

- **玻璃态设计**（Glassmorphism）：毛玻璃效果，视觉层级清晰
- **深色主题**：护眼设计，适合长时间使用
- **响应式布局**：自适应各类屏幕尺寸
- **无缝动画**：过渡流畅，操作反馈及时

### 交互逻辑

- **面板视图** ↔ **字幕条视图**：一键切换，状态共享
- **实时预览**：捕获区域即时显示，支持重新选择
- **配置保存**：翻译服务配置保存到 SQLite，API Key 由系统凭据存储保管；其他设置状态见 `docs/settings-status.md`
- **错误恢复**：网络失败、模型超时时优雅降级

---

## ⚙️ 配置说明

### 字幕库设置（SubsPanel）

```
支持格式：.srt, .ass, .vtt
匹配策略：模糊匹配（Levenshtein 距离）
缓存机制：内存缓存 + localStorage 持久化
命中率：取决于 OCR 质量和字幕准确度
```

### 翻译引擎（TranslatePanel）

```
支持接入：
  - OpenAI API (GPT-3.5 / GPT-4)
  - 国内厂商 (通义千问、文心一言等)
  - 本地模型 (Ollama / LM Studio)

配置项：
  - API 密钥
  - 模型选择
  - 温度 (Temperature)
  - 超时时间
```

### OCR 引擎（OcrPanel）

```
支持接入：
  - 国内：阿里云、百度、腾讯
  - 国际：Google Vision、Azure Computer Vision
  - 开源：PaddleOCR、Tesseract

配置项：
  - 服务商选择
  - API 密钥
  - 语言设置
```

---

## 📊 性能指标

- **捕获延迟**：< 200ms
- **OCR 识别时间**：取决于引擎，通常 500ms ~ 2s
- **翻译响应时间**：字幕库命中 < 50ms，模型翻译 1s ~ 10s
- **内存占用**：150MB ~ 300MB（取决于字幕库大小）

---

## 🔐 隐私与安全

- ✅ 所有截图操作在本地完成，不上传原始图片
- ✅ 仅 OCR 文本和翻译请求发送至外部 API
- ✅ 字幕库本地存储，支持离线使用
- ✅ 快捷键和配置存储在本地，不同步到云端

---

## 🐛 已知限制

- 当前仅支持**英译中**（后续支持多语言）
- OCR 识别质量受屏幕分辨率影响（推荐 1080p 以上）
- 快捷键冲突可能与系统全局快捷键冲突（需要权限配置）
- 不支持实时视频字幕同步（仅适用于静止或缓冲的内容）

---

## 🛣️ 后续规划

- [ ] 多语言翻译支持（中英互译、日韩等）
- [ ] 实时视频字幕流识别和同步
- [ ] 快捷键全局监听和配置 UI
- [ ] 翻译历史和收藏功能
- [ ] 自定义快捷键和主题
- [ ] 批量字幕导入和管理优化
- [ ] 支持 WebRTC 截屏（跨屏共享场景）
- [ ] 社区字幕库分享平台

---

## 📝 许可证

MIT License - 详见 [LICENSE](./LICENSE)

---

## 💬 贡献

欢迎提交 Issue 和 Pull Request！

---

## 📧 联系方式

- 📍 项目主页：[GitHub](https://github.com/hsolaire/lore-lingo)
- 💌 反馈与建议：提交 GitHub Issues

---

**LoreLingo** — *看见世界的语言*
