use std::sync::Arc;

use tauri::{
    Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, State, WebviewUrl,
    WebviewWindow, WebviewWindowBuilder,
};

use crate::{
    domain::captured_layout::{
        CancelCaptureTranslationRequest, CaptureTranslationEvent, CaptureTranslationState,
        CompleteCaptureTranslationRequest, PublicError, SelectorPayload,
        StartCaptureTranslationRequest, StartCaptureTranslationResponse,
    },
    services::capture_translation::{
        CaptureTranslationError, CaptureTranslationService, CAPTURE_TRANSLATION_EVENT,
        SELECTOR_LABEL_PREFIX,
    },
};

#[tauri::command]
pub async fn start_capture_translation(
    app: tauri::AppHandle,
    service: State<'_, Arc<CaptureTranslationService>>,
    request: StartCaptureTranslationRequest,
) -> Result<StartCaptureTranslationResponse, PublicError> {
    close_selector_windows(&app);
    let service = Arc::clone(service.inner());
    let started = tauri::async_runtime::spawn_blocking(move || service.start(request.translation))
        .await
        .map_err(|_| PublicError::from(CaptureTranslationError::BackendFailure))?
        .map_err(PublicError::from)?;

    for spec in &started.windows {
        let window =
            WebviewWindowBuilder::new(&app, &spec.label, WebviewUrl::App("selector.html".into()))
                .title("LoreLingo 截图翻译")
                .decorations(false)
                .transparent(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .visible(false)
                .build()
                .map_err(|_| PublicError::from(CaptureTranslationError::BackendFailure))?;
        window
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
            .map_err(|_| PublicError::from(CaptureTranslationError::BackendFailure))?;
    }
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.hide();
    }
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
    emit_state(
        &app,
        CaptureTranslationEvent {
            session_id: request.session_id.clone(),
            state: CaptureTranslationState::Processing,
            document: None,
            error: None,
        },
    );

    let session_id = request.session_id.clone();
    match service.complete(request).await {
        Ok(document) => {
            let state = if document.blocks.is_empty() {
                CaptureTranslationState::Empty
            } else {
                CaptureTranslationState::Success
            };
            emit_state(
                &app,
                CaptureTranslationEvent {
                    session_id,
                    state,
                    document: Some(document),
                    error: None,
                },
            );
        }
        Err(error) => emit_state(
            &app,
            CaptureTranslationEvent {
                session_id,
                state: CaptureTranslationState::Error,
                document: None,
                error: Some(error.into()),
            },
        ),
    }
    Ok(())
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
