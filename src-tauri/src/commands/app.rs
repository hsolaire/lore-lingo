use std::sync::Arc;

use tauri::{Manager, State};

use crate::{
    domain::{
        app::{BootstrapSnapshot, WindowSummary},
        translation::PublicError,
    },
    services::bootstrap::BootstrapService,
};

#[tauri::command]
pub async fn get_bootstrap_snapshot(
    app: tauri::AppHandle,
    service: State<'_, Arc<BootstrapService>>,
) -> Result<BootstrapSnapshot, PublicError> {
    let windows = app
        .webview_windows()
        .into_iter()
        .map(|(label, window)| WindowSummary {
            label,
            visible: window.is_visible().unwrap_or(false),
            always_on_top: window.is_always_on_top().unwrap_or(false),
        })
        .collect();
    let service = Arc::clone(service.inner());
    tauri::async_runtime::spawn_blocking(move || service.snapshot(windows))
        .await
        .map_err(|_| PublicError::database_unavailable())?
}
