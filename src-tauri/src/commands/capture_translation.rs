use std::sync::Arc;
use std::time::Duration;

use tauri::{
    Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, State, WebviewUrl,
    WebviewWindow, WebviewWindowBuilder,
};

use crate::{
    domain::captured_layout::{
        CancelCaptureTranslationRequest, CaptureTranslationEvent, CaptureTranslationState,
        CaptureWindowTranslationRequest, CompleteCaptureTranslationRequest, PublicError,
        SelectorPayload, StartCaptureTranslationRequest, StartCaptureTranslationResponse,
        TranslationConfig,
    },
    services::capture_translation::{
        CaptureTranslationError, CaptureTranslationService, SelectedCapture,
        CAPTURE_TRANSLATION_EVENT, SELECTOR_LABEL_PREFIX,
    },
};

#[tauri::command]
pub async fn start_capture_translation(
    app: tauri::AppHandle,
    service: State<'_, Arc<CaptureTranslationService>>,
    request: StartCaptureTranslationRequest,
) -> Result<StartCaptureTranslationResponse, PublicError> {
    // 步骤 1. 先关闭旧选择器并隐藏主窗口，保证冻结背景不包含 LoreLingo 自身
    close_selector_windows(&app);
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.hide();
    }
    let service = Arc::clone(service.inner());
    let start_service = Arc::clone(&service);
    let started = match tauri::async_runtime::spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(120));
        start_service.start(request.translation)
    })
    .await
    {
        Ok(Ok(started)) => started,
        Ok(Err(error)) => {
            restore_main_window(&app);
            return Err(error.into());
        }
        Err(_) => {
            restore_main_window(&app);
            return Err(CaptureTranslationError::BackendFailure.into());
        }
    };

    // 步骤 2. 为每块显示器建立临时选择器；任一失败都回滚全部窗口和任务
    for spec in &started.windows {
        let window = match WebviewWindowBuilder::new(
            &app,
            &spec.label,
            WebviewUrl::App("selector.html".into()),
        )
        .title("LoreLingo 截图翻译")
        .decorations(false)
        .transparent(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .visible(false)
        .build()
        {
            Ok(window) => window,
            Err(_) => return rollback_start(&app, &service, &started.session_id),
        };
        if window
            .set_position(Position::Physical(PhysicalPosition::new(
                spec.physical_x,
                spec.physical_y,
            )))
            .and_then(|_| {
                window.set_size(Size::Physical(PhysicalSize::new(
                    spec.physical_width,
                    spec.physical_height,
                )))
            })
            .and_then(|_| window.show())
            .is_err()
        {
            return rollback_start(&app, &service, &started.session_id);
        }
    }

    // 步骤 3. 聚焦第一块显示器并广播唯一的 selecting 状态
    if let Some(first) = started
        .windows
        .first()
        .and_then(|spec| app.get_webview_window(&spec.label))
    {
        let _ = first.set_focus();
    }
    emit_state(
        &app,
        CaptureTranslationEvent {
            session_id: started.session_id.clone(),
            state: CaptureTranslationState::Selecting,
            document: None,
            error: None,
        },
    );
    Ok(StartCaptureTranslationResponse {
        session_id: started.session_id,
    })
}

fn rollback_start(
    app: &tauri::AppHandle,
    service: &Arc<CaptureTranslationService>,
    session_id: &str,
) -> Result<StartCaptureTranslationResponse, PublicError> {
    let _ = service.cancel(session_id);
    close_selector_windows(app);
    restore_main_window(app);
    Err(CaptureTranslationError::BackendFailure.into())
}

#[tauri::command]
pub fn capture_selector_payload(
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
) -> Result<SelectorPayload, PublicError> {
    service
        .selector_payload(window.label())
        .map_err(PublicError::from)
}

#[tauri::command]
pub async fn complete_capture_translation(
    app: tauri::AppHandle,
    service: State<'_, Arc<CaptureTranslationService>>,
    request: CompleteCaptureTranslationRequest,
) -> Result<(), PublicError> {
    close_selector_windows(&app);
    restore_main_window(&app);
    let session_id = request.session_id.clone();
    let service = Arc::clone(service.inner());
    let prepare_service = Arc::clone(&service);
    let selected = tauri::async_runtime::spawn_blocking(move || prepare_service.prepare(&request))
        .await
        .map_err(|_| PublicError::from(CaptureTranslationError::BackendFailure))?
        .map_err(PublicError::from)?;
    process_selected(&app, &service, session_id, selected).await;
    Ok(())
}

async fn process_selected(
    app: &tauri::AppHandle,
    service: &CaptureTranslationService,
    session_id: String,
    selected: SelectedCapture,
) {
    // 步骤 1. 在 OCR 和网络请求开始之前发送真实裁剪图，识别失败时也能预览。
    let preview = selected.preview();
    emit_state(
        app,
        CaptureTranslationEvent {
            session_id: session_id.clone(),
            state: CaptureTranslationState::Processing,
            document: Some(preview.clone()),
            error: None,
        },
    );
    // 步骤 2. 通过任务服务识别与翻译，取消后的结果不能成为有效终态。
    let result = service.complete(&session_id, selected).await;
    if service.check_session(&session_id).is_err() {
        return;
    }
    // 步骤 3. 所有终态保留截图，空文本和远程翻译失败不冒充捕获失败。
    let event = match result {
        Ok(document) => CaptureTranslationEvent {
            session_id,
            state: if document.blocks.is_empty() {
                CaptureTranslationState::Empty
            } else {
                CaptureTranslationState::Success
            },
            document: Some(document),
            error: None,
        },
        Err(error) => CaptureTranslationEvent {
            session_id,
            state: CaptureTranslationState::Error,
            document: Some(preview),
            error: Some(error.into()),
        },
    };
    emit_state(app, event);
}

fn require_main(window: &WebviewWindow) -> Result<(), PublicError> {
    if window.label() != "main" {
        return Err(CaptureTranslationError::UnsupportedCapability.into());
    }
    Ok(())
}

#[tauri::command]
pub fn get_captured_region(
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
) -> Result<Option<crate::domain::captured_layout::CapturedRegion>, PublicError> {
    require_main(&window)?;
    service.region.current().map_err(PublicError::from)
}

#[tauri::command]
pub fn update_captured_region(
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
    request: crate::domain::captured_layout::UpdateCapturedRegionRequest,
) -> Result<crate::domain::captured_layout::CapturedRegion, PublicError> {
    require_main(&window)?;
    service.region.update(request).map_err(PublicError::from)
}

#[tauri::command]
pub async fn captured_region_preview(
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
    revision: u64,
) -> Result<crate::domain::captured_layout::RegionPreview, PublicError> {
    require_main(&window)?;
    let service = Arc::clone(service.inner());
    tauri::async_runtime::spawn_blocking(move || service.region.preview(revision))
        .await
        .map_err(|_| PublicError::from(CaptureTranslationError::BackendFailure))?
        .map_err(PublicError::from)
}

#[tauri::command]
pub async fn translate_captured_region(
    app: tauri::AppHandle,
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
    revision: u64,
    translation: TranslationConfig,
) -> Result<StartCaptureTranslationResponse, PublicError> {
    require_main(&window)?;
    let session_id = service.begin_window().map_err(PublicError::from)?;
    emit_state(
        &app,
        CaptureTranslationEvent {
            session_id: session_id.clone(),
            state: CaptureTranslationState::Processing,
            document: None,
            error: None,
        },
    );
    let service = Arc::clone(service.inner());
    let prepare_service = Arc::clone(&service);
    let selected = tauri::async_runtime::spawn_blocking(move || {
        let image = prepare_service.region.snapshot(revision)?;
        prepare_service.prepare_window(image, translation)
    })
    .await
    .map_err(|_| CaptureTranslationError::BackendFailure)
    .and_then(|result| result);
    if service.check_session(&session_id).is_ok() {
        match selected {
            Ok(selected) => process_selected(&app, &service, session_id.clone(), selected).await,
            Err(error) => emit_state(
                &app,
                CaptureTranslationEvent {
                    session_id: session_id.clone(),
                    state: CaptureTranslationState::Error,
                    document: None,
                    error: Some(error.into()),
                },
            ),
        }
    }
    Ok(StartCaptureTranslationResponse { session_id })
}

fn desktop_error_to_public(error: crate::desktop::DesktopError) -> PublicError {
    PublicError {
        code: serde_json::to_value(error.code)
            .ok()
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "backend_failure".into()),
        message: error.message,
        recoverable: error.recoverable,
        action: error.action.and_then(|action| {
            serde_json::to_value(action)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
        }),
    }
}

#[tauri::command]
pub fn get_capture_translation_config(
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
) -> Result<TranslationConfig, PublicError> {
    if !matches!(window.label(), "main" | "settings") {
        return Err(CaptureTranslationError::UnsupportedCapability.into());
    }
    service.config().map_err(PublicError::from)
}

#[tauri::command]
pub fn save_capture_translation_config(
    window: WebviewWindow,
    service: State<'_, Arc<CaptureTranslationService>>,
    config: TranslationConfig,
) -> Result<TranslationConfig, PublicError> {
    if window.label() != "settings" {
        return Err(CaptureTranslationError::UnsupportedCapability.into());
    }
    service.save_config(config).map_err(PublicError::from)
}

#[tauri::command]
pub async fn capture_window_translation(
    app: tauri::AppHandle,
    window: WebviewWindow,
    desktop: State<'_, Arc<crate::desktop::DesktopService>>,
    service: State<'_, Arc<CaptureTranslationService>>,
    request: CaptureWindowTranslationRequest,
) -> Result<StartCaptureTranslationResponse, PublicError> {
    require_main(&window)?;
    let source = desktop
        .resolve_source(request.catalog_revision, &request.source_id)
        .map_err(desktop_error_to_public)?;
    close_selector_windows(&app);
    let session_id = service.begin_window().map_err(PublicError::from)?;
    emit_state(
        &app,
        CaptureTranslationEvent {
            session_id: session_id.clone(),
            state: CaptureTranslationState::Processing,
            document: None,
            error: None,
        },
    );
    let service = Arc::clone(service.inner());
    let prepare_service = Arc::clone(&service);
    let image = tauri::async_runtime::spawn_blocking(move || {
        let image = crate::desktop::capture_window_image(source)
            .map_err(|error| desktop_error_to_public(error.to_public()))?;
        prepare_service
            .prepare_window(image, request.translation)
            .map_err(PublicError::from)
    })
    .await
    .map_err(|_| PublicError::from(CaptureTranslationError::BackendFailure))?;
    match image {
        Ok(selected) => {
            if service.check_session(&session_id).is_ok() {
                process_selected(&app, &service, session_id.clone(), selected).await;
            }
        }
        Err(error) => {
            if service.check_session(&session_id).is_ok() {
                emit_state(
                    &app,
                    CaptureTranslationEvent {
                        session_id: session_id.clone(),
                        state: CaptureTranslationState::Error,
                        document: None,
                        error: Some(error),
                    },
                );
            }
        }
    }
    Ok(StartCaptureTranslationResponse { session_id })
}

#[tauri::command]
pub fn cancel_capture_translation(
    app: tauri::AppHandle,
    service: State<'_, Arc<CaptureTranslationService>>,
    request: CancelCaptureTranslationRequest,
) -> Result<(), PublicError> {
    service
        .cancel(&request.session_id)
        .map_err(PublicError::from)?;
    close_selector_windows(&app);
    restore_main_window(&app);
    emit_state(
        &app,
        CaptureTranslationEvent {
            session_id: request.session_id,
            state: CaptureTranslationState::Cancelled,
            document: None,
            error: None,
        },
    );
    Ok(())
}

fn emit_state(app: &tauri::AppHandle, event: CaptureTranslationEvent) {
    let _ = app.emit_to("main", CAPTURE_TRANSLATION_EVENT, event);
}

fn close_selector_windows(app: &tauri::AppHandle) {
    for (label, window) in app.webview_windows() {
        if label.starts_with(SELECTOR_LABEL_PREFIX) {
            let _ = window.close();
        }
    }
}

fn restore_main_window(app: &tauri::AppHandle) {
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.show();
        let _ = main.set_focus();
    }
}
