## ADDED Requirements

### Requirement: Platform-correct main window controls
The main window SHALL present and operate controls according to the runtime desktop platform while preserving a shared Vue component contract.

#### Scenario: Main window runs on macOS
- **WHEN** desktop capabilities identify macOS
- **THEN** 主窗口使用 macOS 原生关闭、最小化和全屏交通灯，Vue 工具栏为其预留空间且不渲染重复按钮

### Requirement: 合并悬浮字幕窗口与自动收起工具栏
主面板与悬浮字幕 SHALL 由唯一的 `main` 窗口承载；所有常用操作 SHALL 位于字幕顶部工具栏，工具栏固定状态 SHALL 与窗口置顶状态独立。

#### Scenario: 鼠标移出悬浮字幕
- **WHEN** 鼠标移出，且工具栏未固定、没有键盘焦点或展开的操作面板
- **THEN** 工具栏及 macOS 原生交通灯在短暂延迟后隐藏，字幕内容继续显示，原生窗口缩到内容高度

#### Scenario: 固定工具栏
- **WHEN** 用户启用固定工具栏
- **THEN** 鼠标移开后工具栏保持显示，并且不改变窗口置顶状态

#### Scenario: 键盘或配置交互
- **WHEN** 键盘焦点位于悬浮窗口内，或捕获来源、配置、版面结果或翻译任务正在展开
- **THEN** 工具栏保持可操作，不会中途自动收起

#### Scenario: 旧字幕导航与真实翻译结果
- **WHEN** 旧调用请求打开字幕，或单次截图翻译返回结果
- **THEN** 导航聚焦唯一主窗口，结果直接更新同一窗口的原文和译文，不创建第二个字幕窗口

#### Scenario: Main window runs on Windows
- **WHEN** desktop capabilities identify Windows
- **THEN** minimize and close controls use the Windows placement and interaction convention and macOS-only decoration is not shown

#### Scenario: User drags the custom title bar
- **WHEN** the user drags a non-interactive title-bar area
- **THEN** the native window moves without making title-bar buttons initiate a drag

#### Scenario: 工具栏直接位于字幕顶部
- **WHEN** 工具栏显示
- **THEN** 工具栏直接贴合字幕顶部，共用一个表面和外边框，左侧按翻译选项、捕获窗口、捕获区域、设置排列，窗口置顶和固定工具栏靠右
- **AND** 语言、显示模式、复制和结果操作收纳进翻译选项；模型和服务配置仅通过设置页操作

### Requirement: Content-sized subtitle overlay
The subtitle overlay SHALL use a native window bounded to its visible content rather than an invisible desktop-sized input surface.

#### Scenario: Subtitle overlay is shown
- **WHEN** the user opens the Subtitle Overlay
- **THEN** a transparent always-on-top window is positioned around the subtitle content and unrelated desktop areas remain interactive

#### Scenario: Subtitle content changes height
- **WHEN** wrapping or controls change the measured subtitle content size
- **THEN** the native Subtitle Overlay is resized and clamped without moving off the active monitor

#### Scenario: User closes the Subtitle Overlay
- **WHEN** the Subtitle Overlay is closed
- **THEN** the Subtitle Overlay is hidden or destroyed and does not retain an invisible input-blocking window

### Requirement: Monitor-aware Subtitle Overlay persistence
The system SHALL persist Subtitle Overlay placement relative to a monitor and SHALL recover safely when the stored topology no longer exists.

#### Scenario: Stored monitor remains connected
- **WHEN** the Subtitle Overlay is restored and its stored monitor is available
- **THEN** its position and width are restored and clamped to that monitor's logical work area

#### Scenario: Stored monitor was removed
- **WHEN** the Subtitle Overlay is restored after its stored monitor has been disconnected
- **THEN** it moves to a visible clamped position on the primary monitor

#### Scenario: Scale factor changes
- **WHEN** the Subtitle Overlay moves between monitors with different scale factors
- **THEN** its logical size remains usable and its native physical bounds are recalculated

### Requirement: Temporary per-monitor region selectors
Region selection SHALL use temporary borderless selector windows associated with concrete monitor IDs.

#### Scenario: Region selection begins with multiple monitors
- **WHEN** the user starts region selection with multiple displays connected
- **THEN** a selector surface covers each eligible display and reports monitor-local logical coordinates

#### Scenario: Region selection completes or is cancelled
- **WHEN** the user confirms or cancels selection
- **THEN** every selector window is destroyed before capture starts

### Requirement: Presentation fallback
The application SHALL remain readable and operable when transparent-window or backdrop effects differ between WebView2 and WKWebView.

#### Scenario: Backdrop effect is unavailable
- **WHEN** the runtime cannot render the intended transparent blur effect correctly
- **THEN** the window uses an opaque or translucent fallback with sufficient contrast and unchanged controls

### Requirement: 捕获状态与区域预览
捕获区域入口 SHALL 在真实截图成功后显示已捕获状态；OCR 空结果或翻译失败 SHALL 保留当前截图供预览。

#### Scenario: 区域已捕获但未配置翻译
- **WHEN** 当前区域已有真实截图，远程翻译返回未配置错误
- **THEN** 区域入口继续显示已捕获，悬停或聚焦时展示真实缩略图和捕获尺寸，并明确提示翻译配置错误

#### Scenario: 选择捕获窗口
- **WHEN** 用户从窗口下拉选择一个可捕获来源
- **THEN** 系统以该来源及目录版本启动单次窗口捕获，关闭下拉菜单，并报告真实的处理中、空或错误状态

#### Scenario: 区域实时预览与坐标编辑
- **WHEN** 用户展开已捕获区域面板
- **THEN** 系统按当前显示器局部逻辑坐标捕获最新画面，排除本进程窗口，上一帧完成后间隔 0.5 秒请求下一帧，关闭面板停止后续请求
- **AND** 面板显示可编辑的 X1/X2/Y1/Y2；无效或越界坐标不能应用，合法更新带修订号并丢弃旧帧

#### Scenario: 使用修改后的区域翻译
- **WHEN** 用户应用新坐标并点击翻译此区域
- **THEN** 系统等待预览捕获结束，使用当前修订的选区重新截图，再显式发起 OCR 与翻译，不由预览轮询自动发送识别文本

#### Scenario: 键盘关闭区域编辑
- **WHEN** 用户编辑坐标时按 Escape
- **THEN** 面板关闭，入口恢复焦点，焦点恢复不会重新打开预览

### Requirement: 弹层不扩张字幕背景
翻译、窗口与区域菜单 SHALL 与字幕表面分层，展开菜单不得增加字幕表面的高度、背景或拖动手柄位置。

#### Scenario: 展开与关闭菜单
- **WHEN** 用户展开翻译选项、来源或区域预览
- **THEN** 菜单浮在字幕表面上，宿主只按实际菜单尺寸提供透明空间，不使用固定最小高度撑大字幕背景
- **AND** 菜单关闭后宿主恢复按字幕实际内容计算大小

### Requirement: 轻量字幕缩放提示
悬浮字幕 SHALL 保留两侧宽度拖动命中区，并避免在正文底部常驻绘制粗横杠。

#### Scenario: 悬停字幕内容与缩放边角
- **WHEN** 鼠标进入字幕内容或边角缩放区域
- **THEN** 显示轻量边角提示，边角悬停进一步强调；拖动仍分别使用原生左侧和右侧宽度调整
- **AND** 鼠标离开后提示隐藏，不改变字幕文字和窗口内容尺寸
