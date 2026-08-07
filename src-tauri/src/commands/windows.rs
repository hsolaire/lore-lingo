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
