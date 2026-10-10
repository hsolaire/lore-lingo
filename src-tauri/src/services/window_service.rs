use tauri::{AppHandle, Manager};

use crate::domain::{app::WindowSummary, translation::PublicError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppWindowLabel {
    Main,
    Settings,
}

impl AppWindowLabel {
    const ALL: [Self; 2] = [Self::Main, Self::Settings];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Settings => "settings",
        }
    }
}

impl TryFrom<&str> for AppWindowLabel {
    type Error = PublicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "main" => Ok(Self::Main),
            // 旧调用兼容到同一个悬浮主窗口，不再创建或寻找第二个字幕窗口。
            "subtitle" => Ok(Self::Main),
            "settings" => Ok(Self::Settings),
            _ => Err(PublicError::invalid_window_label()),
        }
    }
}

pub struct WindowService;

impl WindowService {
    pub fn list(app: &AppHandle) -> Vec<WindowSummary> {
        AppWindowLabel::ALL
            .into_iter()
            .filter_map(|label| {
                app.get_webview_window(label.as_str())
                    .map(|window| WindowSummary {
                        label: label.as_str().into(),
                        visible: window.is_visible().unwrap_or(false),
                        always_on_top: window.is_always_on_top().unwrap_or(false),
                    })
            })
            .collect()
    }

    pub fn show_or_focus(app: &AppHandle, label: &str) -> Result<(), PublicError> {
        let label = AppWindowLabel::try_from(label)?;
        let window = app
            .get_webview_window(label.as_str())
            .ok_or_else(PublicError::window_unavailable)?;

        if window.is_minimized().unwrap_or(false) {
            window
                .unminimize()
                .map_err(|_| PublicError::window_unavailable())?;
        }
        window
            .show()
            .map_err(|_| PublicError::window_unavailable())?;
        window
            .set_focus()
            .map_err(|_| PublicError::window_unavailable())
    }

    pub fn hide(app: &AppHandle, label: &str) -> Result<(), PublicError> {
        let label = AppWindowLabel::try_from(label)?;
        let window = app
            .get_webview_window(label.as_str())
            .ok_or_else(PublicError::window_unavailable)?;
        window.hide().map_err(|_| PublicError::window_unavailable())
    }
}

#[cfg(test)]
mod tests {
    use super::AppWindowLabel;

    #[test]
    fn accepts_only_stable_application_window_labels() {
        assert_eq!(AppWindowLabel::try_from("main"), Ok(AppWindowLabel::Main));
        assert_eq!(
            AppWindowLabel::try_from("subtitle"),
            Ok(AppWindowLabel::Main)
        );
        assert_eq!(
            AppWindowLabel::try_from("settings"),
            Ok(AppWindowLabel::Settings)
        );
        assert!(AppWindowLabel::try_from("debug").is_err());
    }
}
