use std::{
    collections::HashMap,
    io::Cursor,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use base64::{engine::general_purpose, Engine as _};
use screenshots::{
    image::{DynamicImage, ImageFormat, RgbaImage},
    Screen,
};

use crate::{
    desktop::ocr,
    domain::captured_layout::{
        build_layout_blocks, logical_to_physical_crop, CompleteCaptureTranslationRequest,
        LayoutDocument, PublicError, SelectorPayload, TranslationConfig, MAX_CAPTURE_PIXELS,
        MAX_CAPTURE_PNG_BYTES,
    },
    services::translation::TranslationClient,
};

pub const CAPTURE_TRANSLATION_EVENT: &str = "capture-translation:state";
pub const SELECTOR_LABEL_PREFIX: &str = "capture-selector-";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureTranslationError {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    UnsupportedCapability,
    InvalidRegion,
    StaleTask,
    CaptureFailure,
    OcrFailure,
    TranslationNotConfigured,
    TranslationUnavailable,
    TranslationProtocol,
    BackendFailure,
}

pub struct CaptureTranslationService {
    next_session: AtomicU64,
    active: Mutex<Option<ActiveTask>>,
    translator: TranslationClient,
}

impl CaptureTranslationService {
    pub fn new() -> Result<Self, CaptureTranslationError> {
        Ok(Self {
            next_session: AtomicU64::new(1),
            active: Mutex::new(None),
            translator: TranslationClient::new()?,
        })
    }

    pub fn start(
        &self,
        translation: TranslationConfig,
    ) -> Result<StartedTask, CaptureTranslationError> {
        #[cfg(not(target_os = "macos"))]
        {
            let _ = translation;
            return Err(CaptureTranslationError::UnsupportedCapability);
        }

        #[cfg(target_os = "macos")]
        {
            let session_id = format!(
                "capture-{}",
                self.next_session.fetch_add(1, Ordering::Relaxed)
            );
            let mut monitors = HashMap::new();
            let mut windows = Vec::new();
            for (index, screen) in Screen::all()
                .map_err(|_| CaptureTranslationError::CaptureFailure)?
                .into_iter()
                .enumerate()
            {
                let info = screen.display_info;
                let image = screen
                    .capture()
                    .map_err(|_| CaptureTranslationError::CaptureFailure)?;
                let pixel_count = u64::from(image.width()) * u64::from(image.height());
                if pixel_count == 0 || pixel_count > MAX_CAPTURE_PIXELS {
                    return Err(CaptureTranslationError::CaptureFailure);
                }
                let png = encode_png(&image)?;
                if png.len() > MAX_CAPTURE_PNG_BYTES {
                    return Err(CaptureTranslationError::CaptureFailure);
                }
                let monitor_id = info.id.to_string();
                let scale_factor = f64::from(info.scale_factor);
                if !scale_factor.is_finite() || scale_factor <= 0.0 {
                    return Err(CaptureTranslationError::CaptureFailure);
                }
                let label = format!("{SELECTOR_LABEL_PREFIX}{session_id}-{index}");
                windows.push(SelectorWindowSpec {
                    label: label.clone(),
                    physical_x: info.x,
                    physical_y: info.y,
                    physical_width: image.width(),
                    physical_height: image.height(),
                });
                monitors.insert(
                    monitor_id.clone(),
                    CapturedMonitor {
                        label,
                        monitor_id,
                        scale_factor,
                        image,
                        png_b64: general_purpose::STANDARD.encode(png),
                    },
                );
            }

            if monitors.is_empty() {
                return Err(CaptureTranslationError::CaptureFailure);
            }
            let mut active = self
                .active
                .lock()
                .map_err(|_| CaptureTranslationError::BackendFailure)?;
            *active = Some(ActiveTask {
                session_id: session_id.clone(),
                translation,
                monitors,
            });
            Ok(StartedTask {
                session_id,
                windows,
            })
        }
    }

    pub fn selector_payload(
        &self,
        label: &str,
    ) -> Result<SelectorPayload, CaptureTranslationError> {
        let active = self
            .active
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        let task = active.as_ref().ok_or(CaptureTranslationError::StaleTask)?;
        let monitor = task
            .monitors
            .values()
            .find(|monitor| monitor.label == label)
            .ok_or(CaptureTranslationError::StaleTask)?;
        Ok(SelectorPayload {
            session_id: task.session_id.clone(),
            monitor_id: monitor.monitor_id.clone(),
            image_png_b64: monitor.png_b64.clone(),
            logical_width: f64::from(monitor.image.width()) / monitor.scale_factor,
            logical_height: f64::from(monitor.image.height()) / monitor.scale_factor,
        })
    }

    pub fn cancel(&self, session_id: &str) -> Result<(), CaptureTranslationError> {
        let mut active = self
            .active
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        let task = active.as_ref().ok_or(CaptureTranslationError::StaleTask)?;
        if task.session_id != session_id {
            return Err(CaptureTranslationError::StaleTask);
        }
        *active = None;
        Ok(())
    }

    pub async fn complete(
        &self,
        request: CompleteCaptureTranslationRequest,
    ) -> Result<LayoutDocument, CaptureTranslationError> {
        let selected = self.take_selected_capture(&request)?;
        let ocr_png = selected.png.clone();
        let observations =
            tauri::async_runtime::spawn_blocking(move || ocr::recognize_text(&ocr_png))
                .await
                .map_err(|_| CaptureTranslationError::OcrFailure)??;
        let mut blocks = build_layout_blocks(observations);

        if !blocks.is_empty() {
            let translations = self
                .translator
                .translate(&blocks, &selected.translation)
                .await?;
            for block in &mut blocks {
                block.translated_text = translations.get(&block.id).cloned();
            }
        }

        Ok(LayoutDocument {
            image_width: selected.width,
            image_height: selected.height,
            image_png_b64: general_purpose::STANDARD.encode(selected.png),
            source_language: selected.translation.source_language,
            target_language: selected.translation.target_language,
            blocks,
        })
    }

    fn take_selected_capture(
        &self,
        request: &CompleteCaptureTranslationRequest,
    ) -> Result<SelectedCapture, CaptureTranslationError> {
        let mut active = self
            .active
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        let task = active.take().ok_or(CaptureTranslationError::StaleTask)?;
        if task.session_id != request.session_id {
            *active = Some(task);
            return Err(CaptureTranslationError::StaleTask);
        }
        let monitor = task
            .monitors
            .get(&request.monitor_id)
            .ok_or(CaptureTranslationError::InvalidRegion)?;
        let crop = logical_to_physical_crop(
            &request.rect,
            monitor.scale_factor,
            monitor.image.width(),
            monitor.image.height(),
        )
        .ok_or(CaptureTranslationError::InvalidRegion)?;
        let image =
            image::imageops::crop_imm(&monitor.image, crop.x, crop.y, crop.width, crop.height)
                .to_image();
        let png = encode_png(&image)?;
        if png.len() > MAX_CAPTURE_PNG_BYTES {
            return Err(CaptureTranslationError::CaptureFailure);
        }
        Ok(SelectedCapture {
            width: image.width(),
            height: image.height(),
            png,
            translation: task.translation,
        })
    }
}

fn encode_png(image: &RgbaImage) -> Result<Vec<u8>, CaptureTranslationError> {
    let mut cursor = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image.clone())
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|_| CaptureTranslationError::CaptureFailure)?;
    Ok(cursor.into_inner())
}

struct ActiveTask {
    session_id: String,
    translation: TranslationConfig,
    monitors: HashMap<String, CapturedMonitor>,
}

struct CapturedMonitor {
    label: String,
    monitor_id: String,
    scale_factor: f64,
    image: RgbaImage,
    png_b64: String,
}

struct SelectedCapture {
    width: u32,
    height: u32,
    png: Vec<u8>,
    translation: TranslationConfig,
}

pub struct StartedTask {
    pub session_id: String,
    pub windows: Vec<SelectorWindowSpec>,
}

pub struct SelectorWindowSpec {
    pub label: String,
    pub physical_x: i32,
    pub physical_y: i32,
    pub physical_width: u32,
    pub physical_height: u32,
}

impl From<CaptureTranslationError> for PublicError {
    fn from(error: CaptureTranslationError) -> Self {
        let (code, message, recoverable) = match error {
            CaptureTranslationError::UnsupportedCapability => (
                "unsupported_capability",
                "当前平台暂不支持结构化截图识别。",
                false,
            ),
            CaptureTranslationError::InvalidRegion => {
                ("invalid_region", "截图区域无效，请重新框选。", true)
            }
            CaptureTranslationError::StaleTask => {
                ("stale_task", "截图任务已失效，请重新开始。", true)
            }
            CaptureTranslationError::CaptureFailure => {
                ("capture_failure", "无法读取截图内容。", true)
            }
            CaptureTranslationError::OcrFailure => ("ocr_failure", "系统文字识别失败。", true),
            CaptureTranslationError::TranslationNotConfigured => (
                "translation_not_configured",
                "请先配置翻译服务、模型和 API Key。",
                true,
            ),
            CaptureTranslationError::TranslationUnavailable => (
                "translation_unavailable",
                "翻译服务暂时不可用，请检查网络和服务配置。",
                true,
            ),
            CaptureTranslationError::TranslationProtocol => (
                "translation_protocol_error",
                "翻译结果结构不完整，未应用部分结果。",
                true,
            ),
            CaptureTranslationError::BackendFailure => {
                ("backend_failure", "截图翻译服务初始化失败。", true)
            }
        };
        Self {
            code: code.into(),
            message: message.into(),
            recoverable,
            action: recoverable.then(|| "retry".into()),
        }
    }
}
