use lore_lingo_lib::desktop::{
    BackendError, CapturePermission, CapturePermissionStatus, CaptureSourceKind,
    DesktopCapabilities, DesktopErrorCode, DesktopService, DiscoveredCaptureSource,
    NativeCaptureSource, PlatformBackend, PlatformKind,
};
use std::sync::{Arc, Mutex};

struct MockBackend {
    discoveries: Mutex<Vec<Result<Vec<DiscoveredCaptureSource>, BackendError>>>,
}

impl MockBackend {
    fn new(discoveries: Vec<Result<Vec<DiscoveredCaptureSource>, BackendError>>) -> Self {
        Self {
            discoveries: Mutex::new(discoveries.into_iter().rev().collect()),
        }
    }
}

impl PlatformBackend for MockBackend {
    fn capabilities(&self) -> DesktopCapabilities {
        DesktopCapabilities::new(PlatformKind::Macos, "aarch64")
            .with_screen_capture(true)
            .with_window_capture(true)
            .with_permission_prompt(true)
    }

    fn capture_permission(&self) -> Result<CapturePermission, BackendError> {
        Ok(CapturePermission::new(
            CapturePermissionStatus::Granted,
            false,
            None,
        ))
    }

    fn request_capture_permission(&self) -> Result<CapturePermission, BackendError> {
        self.capture_permission()
    }

    fn discover_capture_sources(&self) -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
        self.discoveries
            .lock()
            .expect("mock discoveries lock should remain available")
            .pop()
            .expect("test should provide one discovery result per refresh")
    }
}

fn window(title: &str, native_id: u64) -> DiscoveredCaptureSource {
    DiscoveredCaptureSource::new(
        CaptureSourceKind::Window,
        title,
        Some("示例应用".to_string()),
        None,
        true,
        NativeCaptureSource::window(native_id),
    )
}

#[test]
fn refresh_builds_platform_neutral_catalog_with_opaque_ids() {
    let backend = Arc::new(MockBackend::new(vec![Ok(vec![window("资料窗口", 4242)])]));
    let service = DesktopService::new(backend);

    let catalog = service
        .list_capture_sources()
        .expect("source discovery should succeed");

    assert_eq!(catalog.catalog_revision, 1);
    assert_eq!(catalog.sources.len(), 1);
    assert_eq!(catalog.sources[0].catalog_revision, 1);
    assert_eq!(catalog.sources[0].kind, CaptureSourceKind::Window);
    assert_eq!(catalog.sources[0].title, "资料窗口");
    assert_ne!(catalog.sources[0].source_id, "4242");
    assert!(!catalog.sources[0].source_id.contains("4242"));
}

#[test]
fn refresh_invalidates_sources_from_the_previous_revision() {
    let backend = Arc::new(MockBackend::new(vec![
        Ok(vec![window("旧窗口", 100)]),
        Ok(vec![window("新窗口", 100)]),
    ]));
    let service = DesktopService::new(backend);

    let first = service
        .list_capture_sources()
        .expect("first source discovery should succeed");
    let second = service
        .list_capture_sources()
        .expect("second source discovery should succeed");

    assert_eq!(second.catalog_revision, first.catalog_revision + 1);
    let error = service
        .validate_source(first.catalog_revision, &first.sources[0].source_id)
        .expect_err("a source from the replaced catalog must be stale");
    assert_eq!(error.code, DesktopErrorCode::SourceStale);
    service
        .validate_source(second.catalog_revision, &second.sources[0].source_id)
        .expect("a source from the active catalog should resolve");
}

#[test]
fn backend_diagnostics_never_cross_the_serialization_boundary() {
    let backend = Arc::new(MockBackend::new(vec![Err(BackendError::failure(
        "CGWindowListCopyWindowInfo failed for /Users/private/path",
    ))]));
    let service = DesktopService::new(backend);

    let error = service
        .list_capture_sources()
        .expect_err("backend failures should be mapped to a desktop error");
    let json = serde_json::to_string(&error).expect("desktop errors should serialize");

    assert_eq!(error.code, DesktopErrorCode::BackendFailure);
    assert!(json.contains("backend_failure"));
    assert!(!json.contains("CGWindowListCopyWindowInfo"));
    assert!(!json.contains("/Users/private/path"));
}

#[test]
fn capabilities_and_permission_use_the_same_facade_for_every_backend() {
    let backend = Arc::new(MockBackend::new(Vec::new()));
    let service = DesktopService::new(backend);

    let capabilities = service.capabilities();
    let permission = service
        .capture_permission()
        .expect("permission status should come from the backend");

    assert_eq!(capabilities.platform, PlatformKind::Macos);
    assert!(capabilities.window_capture);
    assert!(capabilities.permission_prompt);
    assert_eq!(permission.status, CapturePermissionStatus::Granted);
}
