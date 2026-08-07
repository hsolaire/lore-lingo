use crate::desktop::{
    CapturePermission, CaptureSourceCatalog, DesktopCapabilities, DesktopError, DesktopService,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn get_desktop_capabilities(service: State<'_, Arc<DesktopService>>) -> DesktopCapabilities {
    service.capabilities()
}

#[tauri::command]
pub fn capture_permission_status(
    service: State<'_, Arc<DesktopService>>,
) -> Result<CapturePermission, DesktopError> {
    service.capture_permission()
}

#[tauri::command]
pub fn request_capture_permission(
    service: State<'_, Arc<DesktopService>>,
) -> Result<CapturePermission, DesktopError> {
    service.request_capture_permission()
}

#[tauri::command]
pub fn list_capture_sources(
    service: State<'_, Arc<DesktopService>>,
) -> Result<CaptureSourceCatalog, DesktopError> {
    service.list_capture_sources()
}
