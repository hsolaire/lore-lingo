use std::sync::Arc;

use tauri::State;

use crate::{
    domain::{app::BootstrapSnapshot, translation::PublicError},
    services::{bootstrap::BootstrapService, window_service::WindowService},
};

#[tauri::command]
pub async fn get_bootstrap_snapshot(
    app: tauri::AppHandle,
    service: State<'_, Arc<BootstrapService>>,
) -> Result<BootstrapSnapshot, PublicError> {
    let windows = WindowService::list(&app);
    let service = Arc::clone(service.inner());
    tauri::async_runtime::spawn_blocking(move || service.snapshot(windows))
        .await
        .map_err(|_| PublicError::database_unavailable())?
}
