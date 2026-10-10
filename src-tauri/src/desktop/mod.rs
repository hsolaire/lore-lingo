mod commands;
mod error;
mod models;
pub mod ocr;
mod platform;
mod service;

pub use commands::{
    capture_permission_status, get_desktop_capabilities, list_capture_sources,
    request_capture_permission,
};
pub use error::{BackendError, BackendErrorCode, DesktopError, DesktopErrorCode};
pub use models::{
    CapturePermission, CapturePermissionStatus, CaptureSourceCatalog, CaptureSourceKind,
    CaptureSourceSummary, DesktopCapabilities, LogicalSize, MonitorSummary, PhysicalPoint,
    PhysicalSize, PlatformKind, RecoveryAction,
};
pub use platform::{
    capture_window_image, create_platform_backend, set_window_controls_visible,
    DiscoveredCaptureSource, NativeCaptureSource, PlatformBackend,
};
pub use service::DesktopService;

#[cfg(target_os = "macos")]
pub use platform::capture_display_image;
