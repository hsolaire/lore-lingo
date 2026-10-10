## Context

LoreLingo is a Vue 3 application hosted by Tauri 2. The current Rust entry point mixes Tauri command registration, screenshot capture, Windows window enumeration, and Windows icon extraction in one file. The frontend knows Windows-specific fields such as `exe`, the non-Windows window command returns an empty list, and region selection is simulated. The Subtitle Overlay is a fixed 1920 x 1080 transparent window, which does not model mixed-DPI or changing monitor layouts and can intercept input outside the visible subtitle bar.

The supported release baseline for this change is:

- Windows 10 22H2 and Windows 11 on x86_64, using the MSVC toolchain and WebView2.
- macOS 13 or newer on Apple Silicon and Intel, distributed directly as a signed and notarized Universal 2 application.
- One capture session at a time, capturing one window or a region wholly contained by one monitor.

macOS App Store distribution, Linux, mobile, OCR implementation, translation, and cross-monitor region composition are outside this change.

## Goals / Non-Goals

**Goals:**

- Give Vue one platform-neutral contract for platform capabilities, permissions, screens, windows, and capture sessions.
- Keep native handles, framework types, unsafe code, and platform conditionals inside Rust platform adapters.
- Make source identity and coordinate conversion reliable across refreshes, Retina displays, Windows scaling, secondary displays, and negative virtual-desktop origins.
- Deliver frames to future Rust-side OCR consumers without sending every raw frame through JSON IPC.
- Prevent LoreLingo windows from appearing in captured content or blocking the rest of the desktop.
- Produce and verify native Windows and macOS release artifacts from native CI runners.

**Non-Goals:**

- Selecting one region that spans more than one monitor.
- Capturing DRM-protected or operating-system-protected content.
- Matching every platform's application icon perfectly.
- Implementing OCR, subtitle matching, translation providers, updater infrastructure, or secret storage.
- Supporting the Mac App Store while transparent windows depend on Tauri's macOS private API.

## Decisions

### 1. A Rust facade owns all desktop capabilities

The desktop integration will be organized around a `DesktopService` facade and a target-selected `PlatformBackend` implementation.

```text
Vue composables
    |
    | Tauri commands and state events
    v
commands.rs
    |
    v
DesktopService ---------------- CaptureCoordinator
    |                                  |
    | PlatformBackend                  | bounded latest-frame channel
    v                                  v
platform/windows.rs              future OCR consumer
platform/macos.rs
```

The intended Rust layout is:

```text
src-tauri/src/
  lib.rs
  desktop/
    mod.rs
    commands.rs
    error.rs
    models.rs
    service.rs
    capture.rs
    platform/
      mod.rs
      windows.rs
      macos.rs
```

`lib.rs` will only build application state, register plugins and commands, and start Tauri. Shared models and errors remain serializable and contain no native handles.

Alternative considered: expose separate Windows and macOS commands and branch in Vue. Rejected because it duplicates UI state machines and makes every future desktop feature platform-aware.

### 2. Source identifiers are opaque and catalog-scoped

The shared source model will use a string `source_id`, a `source_kind`, display metadata, an optional icon, and capability flags. It will not expose `HWND`, process paths, Core Graphics objects, or platform framework types.

```text
CaptureSource
  source_id: string
  source_kind: screen | window
  title: string
  application_name?: string
  icon_png_b64?: string
  monitor_id?: string
  capturable: boolean
```

Every discovery response includes a `catalog_revision`. A start request includes the revision and source ID. The service resolves that pair to a native source stored in its catalog. Refreshing discovery replaces the catalog; stale requests fail with `source_stale` instead of accidentally capturing a reused native handle.

Alternative considered: preserve the current numeric `id`. Rejected because handle widths, reuse semantics, and meanings differ by operating system.

### 3. Region requests use monitor-local logical coordinates

A region source is represented by `monitor_id` plus a rectangle in monitor-local logical pixels. The catalog publishes logical size, physical size, scale factor, and virtual-desktop physical origin for each monitor. Vue never converts the selection to physical pixels.

Rust converts a logical rectangle by subtracting no global origin, applying the selected monitor's scale transform, using floor for the physical start and ceil for the physical end, then clamping to physical monitor bounds. This avoids ambiguous global logical coordinates when adjacent monitors use different scale factors.

The region selector creates one temporary selector window per monitor, and each selector reports local CSS coordinates with its monitor ID. The first release keeps the selection inside the monitor where the drag began. Negative virtual-desktop origins are valid metadata and do not affect the local selection contract.

Alternative considered: accept global `x`, `y`, `w`, `h` as the current command does. Rejected because a single global logical coordinate space is not stable under mixed DPI.

### 4. Capture is a Rust-owned session with latest-frame semantics

The command surface will be lifecycle-oriented:

```text
desktop_capabilities() -> DesktopCapabilities
list_capture_sources() -> CaptureSourceCatalog
capture_permission_status() -> CapturePermission
request_capture_permission() -> CapturePermission
start_capture(StartCaptureRequest) -> CaptureSessionInfo
stop_capture(session_id) -> ()
capture_session_status(session_id) -> CaptureSessionStatus
capture_preview(session_id) -> PNG bytes
```

Only one active session is supported initially. Starting a new session stops the previous session after its worker exits. The coordinator owns cancellation, timestamps, frame dimensions, and a capacity-one channel or atomic latest-frame slot. A slow consumer causes intermediate frames to be replaced, not queued. This bounds memory and prevents OCR latency from making capture latency grow without limit.

Raw RGBA frames remain in Rust. The frontend receives state events and requests throttled PNG previews only when needed. A future OCR service consumes the latest Rust frame directly.

Alternative considered: emit every frame as base64 through Tauri events. Rejected because base64 expansion, JSON serialization, and event backlog are unnecessary for the OCR pipeline.

### 5. Native implementations are phased behind the same backend

Windows discovery will retain `EnumWindows` initially but move it into `platform/windows.rs`, improve filtering, and keep process/icon access best-effort. Region capture may use the existing screenshot implementation as a bounded fallback, while window capture and the production streaming path use Windows Graphics Capture. LoreLingo windows will be marked with `WDA_EXCLUDEFROMCAPTURE` when supported.

macOS discovery and permission checks will use Core Graphics/AppKit APIs. The production capture path will use ScreenCaptureKit with a content filter that excludes LoreLingo's own application/windows. Permission requests occur only after an explicit user action and return whether restart or settings navigation is required.

Backend-specific fallback selection is internal. `DesktopCapabilities` reports whether window capture, region capture, self-exclusion, and permission prompting are available so unsupported behavior can be disabled rather than simulated.

Alternative considered: adopt one cross-platform capture crate as the public architecture. Rejected because crate capabilities and performance can change, while the stable project boundary should remain owned by LoreLingo. A crate may still be used inside an adapter.

### 6. Errors are stable, structured, and actionable

Commands return a `DesktopError` with a stable code, safe message, recoverability, and optional recommended action. Initial codes include:

- `permission_denied`
- `permission_restart_required`
- `source_stale`
- `source_not_found`
- `source_unavailable`
- `invalid_region`
- `cross_display_region_unsupported`
- `protected_content`
- `unsupported_capability`
- `backend_failure`

Native handles, paths, API keys, and raw framework errors are logged only in development diagnostics and are not serialized to Vue.

### 7. Presentation uses small persistent windows and temporary selectors

The subtitle overlay becomes a content-sized transparent Tauri window rather than a desktop-sized transparent window. Vue measures content and asks Tauri to resize the native window; dragging uses native window movement. Position is stored as monitor identity plus a normalized horizontal anchor and clamped logical coordinates. If that monitor disappears, the window is moved to and clamped within the primary monitor.

Region selection uses temporary borderless windows sized to each monitor and destroys them when selection finishes or is cancelled. These windows are never left active during capture.

主面板和悬浮字幕合并为唯一的 `main` 悬浮窗口，默认置顶；不再创建独立 `subtitle` 窗口，旧字幕导航映射到 `main`。字幕顶部左侧只保留四个入口：翻译选项下拉、捕获窗口下拉、捕获区域、设置。翻译选项承载语言方向、原文/译文/双语显示、复制和版面结果；模型、服务地址与密钥统一在设置页配置。窗口置顶和固定工具栏独立位于右侧。鼠标移入或键盘焦点进入时显示工具栏，移开 450 毫秒后收起；固定工具栏、展开下拉菜单或区域预览、版面结果和进行中的任务会阻止自动收起。仅持久化工具栏固定偏好，不存储密钥或捕获内容。

工具栏直接位于字幕顶部，与内容共用一个表面和外边框，中间不留间隔。四个入口从左向右排列，窗口置顶和固定工具栏靠右。区域截图完成即显示“已捕获”，不依赖 OCR 或远程翻译成功；悬停或键盘焦点展示真实截图缩略图和物理尺寸。复制及版面操作位于翻译选项内，内容不可用时禁用。macOS 原生交通灯继续位于左上角。

macOS 保留原生交通灯，工具栏为其留位；平台适配器在 AppKit 主线程通过系统按钮的隐藏属性与工具栏同步显示，原生对象不越过平台边界。Windows 在工具栏右侧保留最小化和关闭。Vue 从启动快照获取平台信息。主窗口由内容测量驱动高度变化，工具栏收起后减少空白和输入表面；来源列表、下拉菜单与区域预览浮在字幕表面上；字幕表面高度只由自身内容决定，宿主仅按实际弹层边界留出透明空间，关闭时自动收回，不绘制扩张的整块背景。设置仍是按需显示的独立窗口。截图翻译结果直接更新同一窗口的原文与译文，并保留可展开的版面结果视图。

Alternative considered: keep the fixed transparent Subtitle Overlay. Rejected because it blocks input over invisible areas and assumes one 1920 x 1080 monitor.

### 8. Release builds are native, reproducible, and separate from signing

The repository will standardize on pnpm and a pinned Rust toolchain. Pull-request CI performs frontend checks, Rust tests, and unsigned Tauri builds on native Windows and macOS runners. Release-tag CI adds Windows code signing and macOS signing/notarization using externally configured secrets.

macOS builds produce a Universal 2 artifact containing x86_64 and arm64 slices from a macOS runner. Windows produces x86_64 NSIS/MSI artifacts and validates the WebView2 installation strategy. A Mac build command is never documented as producing a Windows installer, or vice versa.

## Risks / Trade-offs

- [Native streaming APIs increase implementation complexity] -> Keep them behind `PlatformBackend`, land discovery and coordinate tests first, and preserve a region-capture fallback for development.
- [Screen recording permission behavior changes across macOS releases] -> Centralize preflight/request logic, test denied/granted/revoked states, and surface restart/settings actions explicitly.
- [Protected or hardware-overlay video can return blank frames] -> Detect sustained empty frames where possible, return `protected_content` or `source_unavailable`, and document that protected content is unsupported.
- [Mixed-DPI rounding can produce one-pixel seams] -> Use monitor-local coordinates, floor starts, ceil ends, clamp bounds, and cover transforms with table-driven tests.
- [A stale source can refer to a reused native handle] -> Scope source IDs to a catalog revision and resolve them only through the current Rust catalog.
- [Transparent windows can behave differently between WebView2 and WKWebView] -> Provide an opaque visual fallback, keep hit areas bounded to actual windows, and include both engines in smoke tests.
- [Direct distribution with macOS private APIs excludes the App Store] -> Treat signed/notarized direct distribution as the supported channel for this architecture.

## Migration Plan

1. Add shared models, errors, backend trait, and unit tests without changing existing commands.
2. Move current Windows enumeration and screenshot code into the Windows adapter and expose the new discovery commands alongside legacy commands.
3. Add the macOS adapter, permission flow, and source discovery; validate both catalogs before changing Vue.
4. Implement monitor-local region selection and capture sessions, then migrate the frontend composables to the new contract.
5. Replace the fixed Subtitle Overlay with a content-sized window and add self-exclusion.
6. Remove legacy `list_windows`, `list_screens`, and `capture_region` commands after the frontend no longer calls them.
7. Add native CI build jobs, packaging configuration, and the compatibility smoke-test checklist.

Rollback remains possible until step 6 by switching Vue back to legacy commands. After legacy removal, rollback is performed by reverting the change as a unit; no persistent user data migration is involved.

## Implementation Constraints

- The concrete maintained Rust binding for ScreenCaptureKit will be selected by the bounded compile/performance spike; native framework choice remains private to `PlatformBackend` and cannot change the shared contract.
- Global shortcuts are excluded from this compatibility change and will be specified with the later OCR/translation workflow.

### 当前单次 macOS 捕获实现

单次截图翻译在 macOS 14 及以上使用目标限定的 `objc2-screen-capture-kit` 与 `SCScreenshotManager` 捕获显示器或选中窗口，不代表长期连续会话已经完成。窗口请求通过 `sourceId` 与 `catalogRevision` 在 Rust 目录解析，拒绝过期来源。macOS 13 的单帧能力返回显式不支持；Windows、Intel 与连续会话仍需后续原生验证。区域开始前隐藏主窗口，单次截图在 OCR 前发送当前预览；空识别和翻译失败保留预览。

### 区域实时预览与坐标调整

已捕获区域的显示器 ID、尺寸、边界与单调修订号保存在 Rust 进程内。前端面板只保留当前缩略图，每帧完成后 0.5 秒请求下一帧；关闭或切换面板后丢弃在途结果并停止后续轮询。原生捕获仍为有界单帧调用，不代表长期连续 OCR/翻译会话完成。显示器捕获排除本进程窗口，避免递归画面。

X1/Y1 与 X2/Y2 是同一显示器局部的左上、右下逻辑坐标；前后端校验有限数值、显示器边界及最小 4 单位宽高。更新按修订号原子提交，预览失败明确暂停并允许重试。翻译按钮先停预览并等待在途捕获释放，再使用当前坐标重新捕获，不自动调用模型服务。
