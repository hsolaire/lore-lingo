use tauri::AppHandle;

use crate::{
    domain::{app::WindowSummary, translation::PublicError},
    services::window_service::WindowService,
};

#[tauri::command]
pub fn list_app_windows(app: AppHandle) -> Vec<WindowSummary> {
    WindowService::list(&app)
}

#[tauri::command]
pub fn show_or_focus_window(app: AppHandle, label: String) -> Result<(), PublicError> {
    WindowService::show_or_focus(&app, &label)
}

#[tauri::command]
pub fn hide_window(app: AppHandle, label: String) -> Result<(), PublicError> {
    WindowService::hide(&app, &label)
}

#[tauri::command]
pub async fn set_main_window_controls_visible(
    window: tauri::WebviewWindow,
    visible: bool,
) -> Result<(), PublicError> {
    if window.label() != "main" {
        return Err(PublicError::invalid_window_label());
    }
    tauri::async_runtime::spawn_blocking(move || {
        crate::desktop::set_window_controls_visible(window, visible)
    })
    .await
    .map_err(|_| PublicError::window_unavailable())?
}
