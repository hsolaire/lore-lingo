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
    PermissionDenied,
    OcrFailure,
    TranslationNotConfigured,
    TranslationUnavailable,
    TranslationProtocol,
    BackendFailure,
}

pub struct CaptureTranslationService {
    pub region: super::region_capture::RegionCaptureService,
    next_session: AtomicU64,
    active: Mutex<Option<ActiveTask>>,
    translator: TranslationClient,
    config: Mutex<TranslationConfig>,
    current_session: Mutex<Option<String>>,
}

impl CaptureTranslationService {
    pub fn new() -> Result<Self, CaptureTranslationError> {
        Ok(Self {
            region: super::region_capture::RegionCaptureService::default(),
            next_session: AtomicU64::new(1),
            active: Mutex::new(None),
            translator: TranslationClient::new()?,
            config: Mutex::new(TranslationConfig {
                endpoint: String::new(),
                model: String::new(),
                api_key: String::new(),
                source_language: "auto".into(),
                target_language: "zh-CN".into(),
            }),
            current_session: Mutex::new(None),
        })
    }

    pub fn start(
        &self,
        translation: TranslationConfig,
    ) -> Result<StartedTask, CaptureTranslationError> {
        self.clear_active()?;

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
                let image =
                    crate::desktop::capture_display_image(info.id).map_err(|error| match error
                        .to_public()
                        .code
                    {
                        crate::desktop::DesktopErrorCode::PermissionDenied => {
                            CaptureTranslationError::PermissionDenied
                        }
                        crate::desktop::DesktopErrorCode::UnsupportedCapability => {
                            CaptureTranslationError::UnsupportedCapability
                        }
                        _ => CaptureTranslationError::CaptureFailure,
                    })?;
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
            *self
                .current_session
                .lock()
                .map_err(|_| CaptureTranslationError::BackendFailure)? = Some(session_id.clone());
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

    pub fn config(&self) -> Result<TranslationConfig, CaptureTranslationError> {
        self.config
            .lock()
            .map(|config| config.clone())
            .map_err(|_| CaptureTranslationError::BackendFailure)
    }

    pub fn save_config(
        &self,
        mut config: TranslationConfig,
    ) -> Result<TranslationConfig, CaptureTranslationError> {
        // 步骤 1. 统一字段空白，限制配置长度和语言方向。
        config.endpoint = config.endpoint.trim().into();
        config.model = config.model.trim().into();
        config.api_key = config.api_key.trim().into();
        config.source_language = config.source_language.trim().into();
        config.target_language = config.target_language.trim().into();
        if config.endpoint.len() > 2048
            || config.model.len() > 256
            || config.api_key.len() > 16384
            || config.source_language.len() > 128
            || config.target_language.len() > 128
            || config.source_language.is_empty()
            || config.target_language.is_empty()
            || config.target_language == "auto"
        {
            return Err(CaptureTranslationError::TranslationNotConfigured);
        }
        // 步骤 2. 与实际请求共用校验，失败时保留上一次已应用的值。
        super::translation::validated_completion_url(&config)?;
        // 步骤 3. 校验完成后一次性替换共享配置。
        *self
            .config
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)? = config.clone();
        Ok(config)
    }

    pub fn begin_window(&self) -> Result<String, CaptureTranslationError> {
        self.clear_active()?;
        let session = format!(
            "capture-{}",
            self.next_session.fetch_add(1, Ordering::Relaxed)
        );
        *self
            .current_session
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)? = Some(session.clone());
        Ok(session)
    }

    pub fn check_session(&self, session: &str) -> Result<(), CaptureTranslationError> {
        let current = self
            .current_session
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        if current.as_deref() != Some(session) {
            return Err(CaptureTranslationError::StaleTask);
        }
        Ok(())
    }

    pub fn cancel(&self, session_id: &str) -> Result<(), CaptureTranslationError> {
        self.check_session(session_id)?;
        self.clear_active()
    }

    pub fn clear_active(&self) -> Result<(), CaptureTranslationError> {
        *self
            .active
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)? = None;
        *self
            .current_session
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)? = None;
        Ok(())
    }

    pub fn prepare(
        &self,
        request: &CompleteCaptureTranslationRequest,
    ) -> Result<SelectedCapture, CaptureTranslationError> {
        self.check_session(&request.session_id)?;
        self.take_selected_capture(request)
    }

    pub fn prepare_window(
        &self,
        image: RgbaImage,
        translation: TranslationConfig,
    ) -> Result<SelectedCapture, CaptureTranslationError> {
        let pixels = u64::from(image.width()) * u64::from(image.height());
        if pixels == 0 || pixels > MAX_CAPTURE_PIXELS {
            return Err(CaptureTranslationError::CaptureFailure);
        }
        let png = encode_png(&image)?;
        if png.len() > MAX_CAPTURE_PNG_BYTES {
            return Err(CaptureTranslationError::CaptureFailure);
        }
        Ok(SelectedCapture {
            width: image.width(),
            height: image.height(),
            png,
            translation,
        })
    }

    pub async fn complete(
        &self,
        session: &str,
        selected: SelectedCapture,
    ) -> Result<LayoutDocument, CaptureTranslationError> {
        self.check_session(session)?;
        let ocr_png = selected.png.clone();
        let observations =
            tauri::async_runtime::spawn_blocking(move || ocr::recognize_text(&ocr_png))
                .await
                .map_err(|_| CaptureTranslationError::OcrFailure)??;
        self.check_session(session)?;
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

        self.check_session(session)?;
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
        self.region.remember(
            monitor.monitor_id.clone(),
            f64::from(monitor.image.width()) / monitor.scale_factor,
            f64::from(monitor.image.height()) / monitor.scale_factor,
            crate::domain::captured_layout::RegionBounds {
                x1: request.rect.x,
                x2: request.rect.x + request.rect.width,
                y1: request.rect.y,
                y2: request.rect.y + request.rect.height,
            },
        )?;
        Ok(SelectedCapture {
            width: image.width(),
            height: image.height(),
            png,
            translation: task.translation,
        })
    }
}

pub(super) fn encode_png(image: &RgbaImage) -> Result<Vec<u8>, CaptureTranslationError> {
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

pub struct SelectedCapture {
    width: u32,
    height: u32,
    png: Vec<u8>,
    translation: TranslationConfig,
}

impl SelectedCapture {
    pub fn preview(&self) -> LayoutDocument {
        LayoutDocument {
            image_width: self.width,
            image_height: self.height,
            image_png_b64: general_purpose::STANDARD.encode(&self.png),
            source_language: self.translation.source_language.clone(),
            target_language: self.translation.target_language.clone(),
            blocks: vec![],
        }
    }
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
            CaptureTranslationError::PermissionDenied => (
                "permission_denied",
                "请在系统设置中为 LoreLingo 开启屏幕录制权限，重新启动后再捕获。",
                true,
            ),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn translation_config() -> TranslationConfig {
        TranslationConfig {
            endpoint: "http://127.0.0.1:11434/v1".into(),
            model: "test-model".into(),
            api_key: String::new(),
            source_language: "en".into(),
            target_language: "zh-CN".into(),
        }
    }

    fn insert_active_task(service: &CaptureTranslationService, session_id: &str) {
        *service.current_session.lock().expect("写入测试会话") = Some(session_id.into());
        let mut active = service.active.lock().expect("锁定测试任务");
        *active = Some(ActiveTask {
            session_id: session_id.into(),
            translation: translation_config(),
            monitors: HashMap::new(),
        });
    }

    #[test]
    fn cancellation_rejects_stale_sessions_and_releases_the_active_task() {
        let service = CaptureTranslationService::new().expect("创建截图翻译服务");
        insert_active_task(&service, "capture-current");

        assert_eq!(
            service.cancel("capture-stale"),
            Err(CaptureTranslationError::StaleTask)
        );
        assert!(service.active.lock().expect("读取测试任务").is_some());

        service.cancel("capture-current").expect("取消当前任务");
        assert!(service.active.lock().expect("读取测试任务").is_none());
    }

    #[test]
    fn runtime_configuration_is_shared_and_rejects_invalid_input() {
        let service = CaptureTranslationService::new().expect("创建服务");
        let config = translation_config();
        service.save_config(config.clone()).expect("应用配置");
        assert_eq!(service.config().expect("读取配置").model, config.model);
        let mut invalid = config;
        invalid.endpoint = "file:///invalid".into();
        assert!(service.save_config(invalid).is_err());
        assert_eq!(
            service.config().expect("保留原配置").endpoint,
            "http://127.0.0.1:11434/v1"
        );
    }

    #[test]
    fn saving_rejects_unusable_endpoints_and_keeps_applied_configuration() {
        let service = CaptureTranslationService::new().expect("创建服务");
        let original = translation_config();
        service.save_config(original.clone()).expect("应用有效配置");
        for endpoint in [
            "https://",
            "https://api.example.com/v1",
            "https://user:password@api.example.com/v1",
            "http://192.168.1.2/v1",
            "http://127.0.0.1:11434/v1?token=value",
            "http://127.0.0.1:11434/v1#fragment",
        ] {
            let mut invalid = original.clone();
            invalid.endpoint = endpoint.into();
            assert_eq!(
                service.save_config(invalid),
                Err(CaptureTranslationError::TranslationNotConfigured)
            );
            assert_eq!(service.config().expect("读取配置"), original);
        }
        let mut invalid = original.clone();
        invalid.target_language = "auto".into();
        assert!(service.save_config(invalid).is_err());
        assert_eq!(service.config().expect("读取配置"), original);
    }

    #[test]
    fn saving_normalizes_fields_and_accepts_ipv6_loopback_without_a_key() {
        let service = CaptureTranslationService::new().expect("创建服务");
        let mut config = translation_config();
        config.endpoint = " http://[::1]:11434/v1 ".into();
        config.model = " test-model ".into();
        config.source_language = " en ".into();
        config.target_language = " zh-CN ".into();
        let saved = service.save_config(config).expect("IPv6 本地模型无需密钥");
        assert_eq!(saved.endpoint, "http://[::1]:11434/v1");
        assert_eq!(saved.model, "test-model");
        assert_eq!(saved.source_language, "en");
        assert_eq!(saved.target_language, "zh-CN");
    }

    #[test]
    fn selected_capture_preview_survives_independently_of_ocr() {
        let service = CaptureTranslationService::new().expect("创建服务");
        let image = RgbaImage::new(20, 10);
        let selected = service
            .prepare_window(image, translation_config())
            .expect("准备截图");
        let preview = selected.preview();
        assert_eq!((preview.image_width, preview.image_height), (20, 10));
        assert!(!preview.image_png_b64.is_empty());
        assert!(preview.blocks.is_empty());
    }

    #[test]
    fn processing_cancellation_invalidates_future_results() {
        let service = CaptureTranslationService::new().expect("创建服务");
        let session = service.begin_window().expect("开始窗口捕获");
        service.check_session(&session).expect("任务有效");
        service.cancel(&session).expect("取消处理中任务");
        assert_eq!(
            service.check_session(&session),
            Err(CaptureTranslationError::StaleTask)
        );
    }

    #[test]
    fn clearing_active_task_is_idempotent() {
        let service = CaptureTranslationService::new().expect("创建截图翻译服务");
        insert_active_task(&service, "capture-current");

        service.clear_active().expect("清理当前任务");
        service.clear_active().expect("重复清理当前任务");
        assert!(service.active.lock().expect("读取测试任务").is_none());
    }
}
