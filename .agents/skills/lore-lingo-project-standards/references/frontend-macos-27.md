# macOS 27 前端与窗口规范

## 设计目标

- 遵循当前 Apple Human Interface Guidelines，把 macOS 27 的 Liquid Glass 视为功能层材质，不是装饰背景。
- 使用 Liquid Glass 承载工具栏、导航、主要交互控制和短暂弹层；内容区使用标准、稳定、可读的材质。
- 优先形成原生桌面应用的层级、密度和行为，不复制系统应用截图或堆叠玻璃卡片。

## Liquid Glass

- macOS 26 及以上可通过 AppKit `NSGlassEffectView` 获得真实动态玻璃效果；把接入代码放在 Rust macOS adapter 中。
- 在绑定 `NSGlassEffectView` 前完成目标限定的编译与性能验证。Vue 不得持有 AppKit 类型、指针或 native handle。
- macOS 13-25 使用 `NSVisualEffectView` 或受支持的 vibrancy 降级；效果不可用时使用不透明高对比度表面。
- 仅在底层是图片、视频或捕获预览且可读性受控时使用 clear glass；含文本的工具栏、侧栏、弹层使用 regular 风格。
- CSS `backdrop-filter` 只用于浏览器预览和显式降级，不得把它描述成原生 Liquid Glass。
- 遵循减少透明度和增加对比度设置。材质关闭后仍须保留边界、层级和焦点状态。

## 窗口和布局

- 主窗口优先使用原生标题栏和交通灯；必须自定义时，保持左侧位置、命中区域、双击行为和拖拽排除区正确。
- 翻译悬浮窗必须按可见内容调整原生窗口边界，不得创建覆盖桌面的透明输入表面。
- 不把关键操作放在窗口底部。把频繁操作放入顶部工具栏或紧邻相关内容的位置。
- 使用逻辑像素布局并验证 Retina 与多显示器缩放。不得用固定 1920x1080 或设备像素计算 Vue 布局。
- 窗口恢复、显示器变化和缩放变化必须保持翻译悬浮窗可见并限制在工作区内。

## 排版、颜色和控件

- 使用系统字体栈，不打包或伪造 SF Pro。正文、辅助文字和等宽内容使用明确的语义 token。
- 颜色跟随系统 light/dark appearance，并提供高对比度覆盖。只为一个主要操作使用强调色背景。
- 使用 Lucide 作为 WebView 图标；SF Symbols 仅允许出现在目标限定的原生界面中。
- 使用 macOS 风格的紧凑工具栏、分段控制、菜单和圆角，但不通过缩小点击区域追求视觉密度。
- 动画表达状态变化和材质响应；尊重减少动画，避免持续漂移、光晕和无功能的形变。

## 能力与降级

- 通过类型化 capability 报告 Liquid Glass、vibrancy、减少透明度等能力或偏好；具体 DTO 字段必须由 OpenSpec 合约定义，不得在 Vue 中用版本字符串猜测行为。
- 原生材质失败时返回结构化状态并切换降级主题。不得静默显示不可读透明窗口并声称成功。
- 浏览器预览只验证布局、token 和组件状态；真实玻璃、标题栏、窗口阴影和拖拽必须在原生 macOS 上验收。

## 官方基线

- Apple HIG Materials: <https://developer.apple.com/design/human-interface-guidelines/materials>
- Adopting Liquid Glass: <https://developer.apple.com/documentation/technologyoverviews/adopting-liquid-glass>
- NSGlassEffectView: <https://developer.apple.com/documentation/AppKit/NSGlassEffectView>
