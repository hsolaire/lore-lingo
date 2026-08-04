## Why

LensLingo currently has a Tauri desktop shell that compiles on macOS, but its source discovery is Windows-only and its region selection, capture loop, permission flow, and release verification are incomplete. A platform boundary is needed now so Windows and macOS can reach functional parity without spreading operating-system checks through the Vue application.

## What Changes

- Introduce a platform-neutral desktop capability API for runtime platform information, screen and window discovery, capture permission state, and stable capture-source identifiers.
- Replace the current screenshot command contract with explicit monitor-local logical coordinates, validated monitor transforms, structured errors, and capture-session lifecycle commands.
- Add native Windows and macOS adapters behind the shared API while keeping platform-specific handles and frameworks inside Rust.
- Make the main panel, source picker, region selector, and subtitle overlay consume capability data instead of assuming a Windows executable or a fixed 1920 x 1080 desktop.
- Define graceful handling for denied permissions, unavailable or protected content, stale windows, display changes, and unsupported capabilities.
- Establish native CI builds and smoke-test gates for supported Windows and macOS targets.
- Keep OCR and translation outside this change; captured frames are exposed through a stable boundary that later engines can consume.

## Capabilities

### New Capabilities

- `desktop-source-discovery`: Runtime platform capability reporting, screen/window enumeration, permission state, and stable source metadata.
- `screen-capture-session`: Region and window capture sessions with coordinate normalization, lifecycle control, frame delivery, and structured failure handling.
- `desktop-window-presentation`: Platform-correct main-window controls, monitor-aware region selection, and non-blocking subtitle overlay behavior.
- `cross-platform-release-verification`: Reproducible Windows/macOS builds, packaging, signing boundaries, and an automated/manual compatibility matrix.

### Modified Capabilities

None. The repository has no existing OpenSpec capability specifications.

## Impact

- Rust desktop code will be split from `src-tauri/src/lib.rs` into domain services and Windows/macOS adapters.
- Tauri commands and frontend TypeScript models for `list_screens`, `list_windows`, and `capture_region` will be replaced by a versioned, structured contract.
- The Vue capture-source and subtitle composables will consume the shared contract and capability flags.
- Native dependencies will be target-gated; production capture may use Windows Graphics Capture and ScreenCaptureKit while preserving a bounded fallback for initial delivery.
- Tauri configuration, macOS usage descriptions/entitlements, Windows bundle settings, CI workflows, and release documentation will change.
- Existing unrelated database migration work is not affected.
