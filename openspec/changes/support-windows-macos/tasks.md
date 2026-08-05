## 1. Toolchain and Contract Baseline

- [ ] 1.1 Add a pinned `rust-toolchain.toml`, standardize project scripts on pnpm, and document the supported Windows/macOS targets without modifying unrelated database dependencies
- [ ] 1.2 Add Rust unit-test modules and frontend test scripts needed by later coordinate, catalog, and composable tests
- [ ] 1.3 Record the existing frontend build and native macOS/Windows compile baselines in the compatibility checklist

## 2. Shared Desktop Capability Layer

- [ ] 2.1 Create the `desktop` module layout and reduce `lib.rs` to application setup, managed state, and command registration
- [ ] 2.2 Define serializable platform, permission, monitor, capture-source, catalog, session, frame-metadata, and capability models using opaque string IDs
- [ ] 2.3 Define `DesktopError` codes, recoverability, safe messages, and optional recovery actions, with serialization tests that exclude native details
- [ ] 2.4 Define the target-selected `PlatformBackend` contract and construct it behind a platform-neutral `DesktopService`
- [ ] 2.5 Implement monitor-local logical-to-physical rectangle conversion with table-driven tests for fractional scaling, clamping, and negative monitor origins

## 3. Discovery Catalog and Commands

- [ ] 3.1 Implement managed source catalogs with monotonic revisions and private mappings from opaque IDs to native source references
- [ ] 3.2 Implement capability, permission-status, permission-request, and source-discovery Tauri commands against `DesktopService`
- [ ] 3.3 Reject stale catalog revisions and missing sources with stable errors, including tests for native-handle reuse protection
- [ ] 3.4 Filter LoreLingo's own main, subtitle, and selector windows from user-selectable discovery results

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
- [ ] 6.3 Implement macOS monitor and on-screen window discovery with best-effort application names/icons and LoreLingo self-filtering
- [ ] 6.4 Complete a bounded ScreenCaptureKit spike that proves region/window capture, content filtering, cancellation, and Intel/Apple Silicon compilation
- [ ] 6.5 Implement macOS region and window capture sessions behind `PlatformBackend`, including frame conversion and structured stale/protected-source errors
- [ ] 6.6 Exclude LoreLingo application windows with ScreenCaptureKit content filters and verify permission revoke/regrant behavior

## 7. Frontend Contract Migration

- [ ] 7.1 Add a typed desktop client and capability composable that is the only frontend module allowed to invoke desktop capability commands
- [ ] 7.2 Replace `WindowInfo` and Windows `exe` assumptions with shared `CaptureSource` models, permission states, catalog revisions, and actionable errors
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
