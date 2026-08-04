## Context

LensLingo is a Vue 3 application hosted by Tauri 2. The current Rust entry point mixes Tauri command registration, screenshot capture, Windows window enumeration, and Windows icon extraction in one file. The frontend knows Windows-specific fields such as `exe`, the non-Windows window command returns an empty list, and region selection is simulated. The Subtitle Overlay is a fixed 1920 x 1080 transparent window, which does not model mixed-DPI or changing monitor layouts and can intercept input outside the visible subtitle bar.

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
- Prevent LensLingo windows from appearing in captured content or blocking the rest of the desktop.
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

Windows discovery will retain `EnumWindows` initially but move it into `platform/windows.rs`, improve filtering, and keep process/icon access best-effort. Region capture may use the existing screenshot implementation as a bounded fallback, while window capture and the production streaming path use Windows Graphics Capture. LensLingo windows will be marked with `WDA_EXCLUDEFROMCAPTURE` when supported.

macOS discovery and permission checks will use Core Graphics/AppKit APIs. The production capture path will use ScreenCaptureKit with a content filter that excludes LensLingo's own application/windows. Permission requests occur only after an explicit user action and return whether restart or settings navigation is required.

Backend-specific fallback selection is internal. `DesktopCapabilities` reports whether window capture, region capture, self-exclusion, and permission prompting are available so unsupported behavior can be disabled rather than simulated.

Alternative considered: adopt one cross-platform capture crate as the public architecture. Rejected because crate capabilities and performance can change, while the stable project boundary should remain owned by LensLingo. A crate may still be used inside an adapter.

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

The main window keeps custom decorations, but controls are platform-correct: macOS traffic controls on the left and Windows minimize/close controls on the right. Behavior uses Tauri APIs, while Vue obtains platform data from `DesktopCapabilities` rather than user-agent parsing.

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
