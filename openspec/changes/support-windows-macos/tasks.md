> 2026-08-05 基线复核：本阶段新增的启动 DTO、SQLite 和启动快照不等同于跨平台 DesktopService、来源目录或平台适配器，因此除已有的 8.2 外不新增完成项。
>
> 2026-08-07 来源目录阶段：已接入平台中立的 `DesktopService`、目标平台 `PlatformBackend`、权限与来源目录命令，并完成 Windows/macOS 窗口发现代码和前端来源选择迁移。本阶段不包含 WGC/ScreenCaptureKit 捕获会话、真实区域选择或双平台实机验收；当时 `windowCapture` 为 `false`。2026-10-10 已增加 macOS 14+ 的单次窗口截图能力，尚未完成连续捕获会话。

## 1. Toolchain and Contract Baseline

- [ ] 1.1 Add a pinned `rust-toolchain.toml`, standardize project scripts on pnpm, and document the supported Windows/macOS targets without modifying unrelated database dependencies
- [ ] 1.2 Add Rust unit-test modules and frontend test scripts needed by later coordinate, catalog, and composable tests
- [ ] 1.3 Record the existing frontend build and native macOS/Windows compile baselines in the compatibility checklist

## 2. Shared Desktop Capability Layer

- [ ] 2.1 Create the `desktop` module layout and reduce `lib.rs` to application setup, managed state, and command registration
- [ ] 2.2 Define serializable platform, permission, monitor, capture-source, catalog, session, frame-metadata, and capability models using opaque string IDs
- [x] 2.3 Define `DesktopError` codes, recoverability, safe messages, and optional recovery actions, with serialization tests that exclude native details
- [x] 2.4 Define the target-selected `PlatformBackend` contract and construct it behind a platform-neutral `DesktopService`
- [ ] 2.5 Implement monitor-local logical-to-physical rectangle conversion with table-driven tests for fractional scaling, clamping, and negative monitor origins

## 3. Discovery Catalog and Commands

- [x] 3.1 Implement managed source catalogs with monotonic revisions and private mappings from opaque IDs to native source references
- [x] 3.2 Implement capability, permission-status, permission-request, and source-discovery Tauri commands against `DesktopService`
- [x] 3.3 Reject stale catalog revisions and missing sources with stable errors, including tests for native-handle reuse protection
- [x] 3.4 Filter LoreLingo's own main, subtitle, and selector windows from user-selectable discovery results

## 4. Capture Coordinator

- [ ] 4.1 Implement capture-session state, cancellation, terminal status, and single-active-session replacement semantics
- [ ] 4.2 Implement a capacity-one latest-frame store with bounded memory and tests proving slow consumers do not create a backlog
- [ ] 4.3 Implement start, stop, and status commands with cleanup on explicit stop, replacement, application exit, and backend failure
- [ ] 4.4 Implement throttled PNG preview responses from the latest Rust-side frame without continuous base64 event delivery
- [ ] 4.5 Emit low-volume capture state changes to Vue and keep raw frames available to a future in-process OCR consumer

## 5. Windows Adapter

- [ ] 5.1 Move existing Win32 screen/window enumeration and icon extraction into `desktop/platform/windows.rs` while preserving best-effort metadata behavior
- [ ] 5.2 Implement Windows capability and permission reporting, including per-source access failures and elevated-window limitations
- [ ] 5.3 Replace global-coordinate region capture with the tested monitor-local transform and verify 100, 150, and 200 percent scaling on primary and secondary displays
- [ ] 5.4 Complete a bounded Windows Graphics Capture spike that proves window capture, cancellation, frame format conversion, and supported minimum Windows versions
- [ ] 5.5 Implement Windows window capture sessions behind `PlatformBackend` and map closed, unavailable, blank, and protected sources to structured errors
- [ ] 5.6 Apply `WDA_EXCLUDEFROMCAPTURE` to LoreLingo windows where supported and report accurate self-exclusion capability when unavailable

## 6. macOS Adapter

- [ ] 6.1 Add target-gated Core Graphics/AppKit and ScreenCaptureKit bindings without introducing macOS dependencies into Windows builds
- [ ] 6.2 Implement screen-capture permission preflight, explicit request, denied/settings, and restart-required states
- [x] 6.3 Implement macOS monitor and on-screen window discovery with best-effort application names/icons and LoreLingo self-filtering
- [ ] 6.4 Complete a bounded ScreenCaptureKit spike that proves region/window capture, content filtering, cancellation, and Intel/Apple Silicon compilation
- [ ] 6.5 Implement macOS region and window capture sessions behind `PlatformBackend`, including frame conversion and structured stale/protected-source errors
- [ ] 6.6 Exclude LoreLingo application windows with ScreenCaptureKit content filters and verify permission revoke/regrant behavior

## 7. Frontend Contract Migration

- [ ] 7.1 Add a typed desktop client and capability composable that is the only frontend module allowed to invoke desktop capability commands
- [x] 7.2 Replace `WindowInfo` and Windows `exe` assumptions with shared `CaptureSource` models, permission states, catalog revisions, and actionable errors
- [ ] 7.3 Migrate the source picker to refresh the catalog, disable unsupported choices, preserve best-effort metadata, and recover from stale selections
- [ ] 7.4 Replace simulated region selection with temporary per-monitor selector windows that return monitor-local logical rectangles and clamp drags to one monitor
- [ ] 7.5 Connect source selection to capture-session start/stop/status and remove simulated success timers and hard-coded region metadata

## 8. Cross-Platform Window Presentation

- [ ] 8.1 Render and operate macOS and Windows custom title-bar controls from runtime capability data, with native drag behavior and accessible hit targets
- [x] 8.2 Replace the fixed 1920 x 1080 subtitle overlay with a content-sized native window that resizes to measured content and leaves unrelated desktop areas interactive
- [ ] 8.3 Persist subtitle placement by monitor and normalized anchor, clamp it after scale/topology changes, and recover to the primary monitor when needed
- [ ] 8.4 Add opaque/translucent visual fallbacks for unsupported or inconsistent transparency/backdrop behavior in WebView2 and WKWebView
- [ ] 8.5 Remove temporary selector windows before capture starts and verify that no invisible LoreLingo window continues intercepting desktop input

## 9. Legacy Removal and Regression Coverage

- [ ] 9.1 Remove legacy `list_windows`, `list_screens`, and `capture_region` commands after all frontend callers use the new contract
- [ ] 9.2 Remove dead fixed-region, fixed Subtitle Overlay, macOS-only OCR status, and macOS-only shortcut presentation assumptions from compatibility-critical UI
- [ ] 9.3 Add Rust regression tests for catalog revisions, coordinate transforms, session replacement, bounded frames, and error mapping
- [ ] 9.4 Add frontend tests for permission states, platform controls, stale source recovery, selector cancellation, and Subtitle Overlay topology restoration
- [ ] 9.5 Run frontend build/tests, Rust fmt/clippy/tests, and native Tauri development smoke tests on both operating systems

## 10. Native CI, Packaging, and Release Verification

- [ ] 10.1 Add pull-request CI jobs on native Windows and macOS runners for pnpm checks, Rust tests, and unsigned Tauri builds
- [ ] 10.2 Build and verify a macOS Universal 2 artifact containing Intel and Apple Silicon slices
- [ ] 10.3 Configure Windows x86_64 NSIS/MSI packaging and document/test the WebView2 bootstrap strategy
- [ ] 10.4 Add credential-gated Windows signing and macOS signing/notarization jobs for authorized release tags without exposing secrets to pull requests
- [ ] 10.5 Create and execute the release smoke-test matrix for scaling, multiple displays, permission transitions, capture failures, Subtitle Overlay exclusion, installation, upgrade, and uninstall
- [ ] 10.6 Correct README and development documentation so host-local builds are never described as producing another operating system's bundles

## 11. 合并悬浮字幕与顶部工具栏

- [x] 11.1 将主面板与悬浮字幕合并为唯一 main 窗口，旧字幕导航兼容到 main
- [x] 11.2 将捕获、翻译、方向、配置、设置、版面结果、置顶和工具栏固定放到顶部
- [x] 11.3 支持移入显示、移开延迟收起、固定偏好恢复和键盘焦点保持
- [x] 11.4 macOS 原生交通灯与工具栏同步显示，窗口按内容调整高度
- [x] 11.5 完成构建、单元测试和 macOS arm64 单 Retina 原生交互验收；Windows 与完整辅助功能/DPI 矩阵另行执行
- [x] 11.6 工具栏直接贴合字幕顶部，左侧翻译入口与右侧窗口操作保持单行，空内容时按钮位置稳定

- [x] 11.7 收敛为翻译选项、窗口、区域和设置四个入口，区域捕获状态与可访问缩略图独立于翻译结果
- [x] 11.8 设置页为区域与窗口任务提供同一个进程内服务配置，密钥不写入浏览器存储或数据库
- [x] 11.9 接入 macOS 14+ 单帧显示器/窗口捕获，目录版本校验和原生权限失败提示；长期会话与其他平台仍未完成

- [x] 11.10 已捕获区域支持面板内有界实时预览、X1/X2/Y1/Y2 编辑、修订号校验与同一区域重新捕获；macOS arm64 单 Retina 实测刷新、坐标尺寸、非法输入与 Escape 焦点恢复

- [x] 11.11 将菜单与字幕背景分层，移除展开菜单时的固定最小高度，按实际弹层尺寸留透明空间并在关闭后回收

### 11.11 验证记录

- `pnpm test`：19 个文件、66 项通过；`pnpm build`、Rust 格式/编译检查和 `git diff --check` 通过。
- macOS 原生 WKWebView 热更新验证翻译菜单和实时区域面板：背景与拖动手柄保持字幕内容高度，菜单外空间透明；Escape 关闭后回到紧凑窗口。
- 目标语言菜单使用紧凑滚动列表，原生滚动条可到达最后的韩语选项，没有改变用户当前语言或已应用区域坐标；Windows 与完整 DPI/辅助功能矩阵未验证。
