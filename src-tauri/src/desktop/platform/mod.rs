use crate::desktop::{
    BackendError, CapturePermission, CaptureSourceKind, DesktopCapabilities, MonitorSummary,
};
use screenshots::Screen;
use std::sync::Arc;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
mod macos_capture;
#[cfg(target_os = "macos")]
mod macos_window;

pub fn set_window_controls_visible(
    window: tauri::WebviewWindow,
    visible: bool,
) -> Result<(), crate::domain::translation::PublicError> {
    #[cfg(target_os = "macos")]
    {
        macos_window::set_controls_visible(window, visible)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (window, visible);
        Ok(())
    }
}
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod unsupported;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeCaptureSource {
    Screen(u64),
    Window(u64),
}

impl NativeCaptureSource {
    pub fn screen(native_id: u64) -> Self {
        Self::Screen(native_id)
    }

    pub fn window(native_id: u64) -> Self {
        Self::Window(native_id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiscoveredCaptureSource {
    pub kind: CaptureSourceKind,
    pub title: String,
    pub application_name: Option<String>,
    pub icon_data_url: Option<String>,
    pub monitor: Option<MonitorSummary>,
    pub capturable: bool,
    pub native_source: NativeCaptureSource,
}

impl DiscoveredCaptureSource {
    pub fn new(
        kind: CaptureSourceKind,
        title: impl Into<String>,
        application_name: Option<String>,
        icon_data_url: Option<String>,
        capturable: bool,
        native_source: NativeCaptureSource,
    ) -> Self {
        Self {
            kind,
            title: title.into(),
            application_name,
            icon_data_url,
            monitor: None,
            capturable,
            native_source,
        }
    }

    pub fn with_monitor(mut self, monitor: MonitorSummary) -> Self {
        self.monitor = Some(monitor);
        self
    }
}

pub trait PlatformBackend: Send + Sync {
    fn capabilities(&self) -> DesktopCapabilities;
    fn capture_permission(&self) -> Result<CapturePermission, BackendError>;
    fn request_capture_permission(&self) -> Result<CapturePermission, BackendError>;
    fn discover_capture_sources(&self) -> Result<Vec<DiscoveredCaptureSource>, BackendError>;
}

pub(crate) fn should_include_window(
    title: &str,
    owner_pid: u32,
    self_pid: u32,
    visible: bool,
    layer: i32,
) -> bool {
    visible && layer == 0 && owner_pid != self_pid && !title.trim().is_empty()
}

pub(crate) fn build_monitor_summary(
    monitor_id: impl Into<String>,
    x: i32,
    y: i32,
    logical_width: u32,
    logical_height: u32,
    scale_factor: f64,
    primary: bool,
) -> MonitorSummary {
    MonitorSummary {
        monitor_id: monitor_id.into(),
        primary,
        logical_size: crate::desktop::LogicalSize {
            width: f64::from(logical_width),
            height: f64::from(logical_height),
        },
        physical_size: crate::desktop::PhysicalSize {
            width: (f64::from(logical_width) * scale_factor).round() as u32,
            height: (f64::from(logical_height) * scale_factor).round() as u32,
        },
        scale_factor,
        physical_origin: crate::desktop::PhysicalPoint {
            x: (f64::from(x) * scale_factor).round() as i32,
            y: (f64::from(y) * scale_factor).round() as i32,
        },
    }
}

pub(crate) fn discover_screens() -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
    let screens = Screen::all()
        .map_err(|error| BackendError::failure(format!("screen discovery failed: {error}")))?;
    Ok(screens
        .into_iter()
        .map(|screen| {
            let info = screen.display_info;
            let monitor_id = format!("display-{}", info.id);
            let title = if info.is_primary {
                "主显示器".to_string()
            } else {
                format!("显示器 {}", info.id)
            };
            DiscoveredCaptureSource::new(
                CaptureSourceKind::Screen,
                title,
                None,
                None,
                true,
                NativeCaptureSource::screen(u64::from(info.id)),
            )
            .with_monitor(build_monitor_summary(
                monitor_id,
                info.x,
                info.y,
                info.width,
                info.height,
                f64::from(info.scale_factor),
                info.is_primary,
            ))
        })
        .collect())
}

pub fn create_platform_backend() -> Arc<dyn PlatformBackend> {
    #[cfg(target_os = "macos")]
    {
        Arc::new(macos::MacosBackend::default())
    }

    #[cfg(target_os = "windows")]
    {
        Arc::new(windows::WindowsBackend)
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Arc::new(unsupported::UnsupportedBackend)
    }
}

#[cfg(target_os = "macos")]
pub fn capture_display_image(id: u32) -> Result<screenshots::image::RgbaImage, BackendError> {
    macos_capture::capture(macos_capture::CaptureTarget::Display(id))
}

pub fn capture_window_image(
    source: NativeCaptureSource,
) -> Result<screenshots::image::RgbaImage, BackendError> {
    #[cfg(target_os = "macos")]
    if let NativeCaptureSource::Window(id) = source {
        let id = u32::try_from(id).map_err(|_| BackendError::failure("invalid capture target"))?;
        return macos_capture::capture(macos_capture::CaptureTarget::Window(id));
    }
    #[cfg(not(target_os = "macos"))]
    let _ = source;
    Err(BackendError::new(
        crate::desktop::BackendErrorCode::UnsupportedCapability,
        "single-frame window capture unavailable",
    ))
}

#[cfg(test)]
mod tests {
    use super::should_include_window;

    #[test]
    fn source_filter_keeps_visible_titled_application_windows() {
        assert!(should_include_window("资料窗口", 12, 99, true, 0));
    }

    #[test]
    fn source_filter_excludes_self_empty_hidden_and_nonstandard_layers() {
        assert!(!should_include_window("LoreLingo", 99, 99, true, 0));
        assert!(!should_include_window("", 12, 99, true, 0));
        assert!(!should_include_window("资料窗口", 12, 99, false, 0));
        assert!(!should_include_window("菜单栏", 12, 99, true, 25));
    }

    #[test]
    fn monitor_metadata_preserves_negative_origin_and_scale() {
        let monitor = super::build_monitor_summary("display-7", -1280, 0, 1280, 720, 2.0, false);

        assert_eq!(monitor.monitor_id, "display-7");
        assert_eq!(monitor.logical_size.width, 1280.0);
        assert_eq!(monitor.physical_size.width, 2560);
        assert_eq!(monitor.physical_origin.x, -2560);
        assert_eq!(monitor.scale_factor, 2.0);
        assert!(!monitor.primary);
    }
}
