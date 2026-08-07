use crate::desktop::{
    BackendError, BackendErrorCode, CapturePermission, CapturePermissionStatus,
    DesktopCapabilities, DiscoveredCaptureSource, PlatformBackend, PlatformKind,
};

pub struct UnsupportedBackend;

impl PlatformBackend for UnsupportedBackend {
    fn capabilities(&self) -> DesktopCapabilities {
        DesktopCapabilities::new(PlatformKind::Unsupported, std::env::consts::ARCH)
    }

    fn capture_permission(&self) -> Result<CapturePermission, BackendError> {
        Ok(CapturePermission::new(
            CapturePermissionStatus::Unsupported,
            false,
            None,
        ))
    }

    fn request_capture_permission(&self) -> Result<CapturePermission, BackendError> {
        Err(BackendError::new(
            BackendErrorCode::UnsupportedCapability,
            "desktop capture is unsupported on this platform",
        ))
    }

    fn discover_capture_sources(&self) -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
        Err(BackendError::new(
            BackendErrorCode::UnsupportedCapability,
            "capture source discovery is unsupported on this platform",
        ))
    }
}
