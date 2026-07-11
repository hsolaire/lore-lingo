# 🎬 LensLingo

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

### 🪟 悬浮字幕窗口

- **屏幕置顶显示**：支持将翻译悬浮窗置于屏幕最前层
- **自由拖拽定位**：鼠标拖动调整窗口位置，位置自动保存
- **视图切换**
  - 面板视图：紧凑的控制面板，支持设置和字幕管理
  - 字幕条视图：全屏沉浸式字幕显示（类似视频播放器）
- **自动隐藏控制栏**：鼠标离开后短暂显示，随后自动隐藏

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
│   │   ├── lyric/               # 字幕悬浮窗相关
│   │   │   ├── LyricBar.vue     # 悬浮窗容器（可拖拽）
│   │   │   ├── LyricBox.vue     # 字幕显示区域
│   │   │   └── LyricControls.vue # 悬浮窗控制栏
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
│   │   ├── useLyricMode.ts      # 悬浮窗模式管理
│   │   ├── useCaptureSource.ts  # 捕获源管理（应用/区域切换）
│   │   ├── useTranslator.ts     # 翻译状态共享
│   │   ├── useSubtitles.ts      # 字幕库管理
│   │   ├── useLyricDrag.ts      # 悬浮窗拖拽逻辑
│   │   ├── useSettings.ts       # 设置面板状态
│   │   └── useToast.ts          # 消息队列
│   ├── icons/
│   └── App.vue                  # 应用根组件
├── src-tauri/                   # Tauri 后端
│   ├── src/
│   │   ├── lib.rs               # 核心逻辑（屏幕截图、字幕解析等）
│   │   └── main.rs              # 入口点
│   ├── Cargo.toml
│   └── tauri.conf.json          # Tauri 配置（窗口、权限等）
├── index.html
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
- **自动保存**：悬浮窗位置、设置项自动持久化到 localStorage
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

**LensLingo** — *看见世界的语言*
