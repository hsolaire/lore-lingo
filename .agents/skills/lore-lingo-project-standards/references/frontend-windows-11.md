# Windows 11 前端与窗口规范

## 设计目标

- 遵循 Fluent 2 和 Windows 11 桌面交互，不把 macOS Liquid Glass 主题简单换色后用于 Windows。
- 使用同一套 Vue 业务组件和语义设计令牌，通过 Windows 主题覆盖改变材质、字体、圆角、间距和状态。
- 保持 Windows 10 22H2 可用；Windows 11 效果不可用时必须提供明确降级。

## 材质

- 使用 Mica 表达持久窗口背景和应用层级，使用 Acrylic 表达菜单、弹层等短暂表面；不要在所有内容区叠加透明材质。
- 把 DWM、窗口背景和原生材质接入放在 Rust Windows 适配器中。Vue 只消费能力数据与语义状态。
- Windows 10、远程桌面、节能模式或效果关闭时使用不透明表面；保证文字、边界和焦点环仍然清晰。
- CSS 模糊只作为 WebView 内局部效果或预览降级，不得冒充系统 Mica/Acrylic。

## 窗口和布局

- 主窗口优先使用系统标题栏按钮；自定义时在右侧实现最小化和关闭，并匹配 Windows 命中区域与拖拽排除区。
- 不显示 macOS 交通灯、SF Symbols 或 macOS 独有的控制排列。
- 使用稳定的 4px/8px 圆角层级、1px 描边和 Fluent 层级设计令牌；避免所有控件都使用胶囊形状。
- 使用逻辑像素并验证 100%、125%、150%、175% 和 200% DPI。布局不得因文字缩放或系统字体替换而溢出。
- 悬浮字幕保持内容尺寸、置顶和可交互边界准确，不得阻塞不可见桌面区域。

## 排版、颜色和交互

- 使用 `Segoe UI Variable`, `Segoe UI`, system-ui 的系统字体栈，不打包系统字体。
- 使用系统强调色时保持文本对比度；错误、警告和成功状态同时使用图标或文字。
- 为键盘用户提供符合 Fluent 的清晰焦点环；支持 Tab、方向键、Enter、Space 和 Escape 的预期行为。
- 悬停和按下状态不得改变控件尺寸。动画保持克制，并尊重 Windows 动画与辅助功能设置。
- 工具栏使用图标或图标加短标签；陌生图标必须有中文工具提示与无障碍名称。

## 能力与降级

- 通过类型化能力数据报告 Mica、Acrylic、系统背景材质和辅助功能偏好；具体数据传输对象字段必须由 OpenSpec 合约定义，不得解析用户代理字符串。
- 材质或窗口 API 不支持时返回显式能力状态，让 Windows 主题选择降级设计令牌。
- 浏览器预览不代表 WebView2、DWM、标题栏按钮或 DPI 原生验收通过。

## 官方基线

- Fluent 2：<https://fluent2.microsoft.design/>
- Fluent 2 设计令牌：<https://fluent2.microsoft.design/design-tokens>
- Fluent 2 排版：<https://fluent2.microsoft.design/typography>
