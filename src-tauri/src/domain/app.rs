use serde::{Deserialize, Serialize};

use super::{
    history::HistorySummary,
    settings::{
        CaptureSettingsSummary, ModelProfileSummary, SettingsSummary, ShortcutBindingSummary,
    },
    translation::{ActiveSessionSummary, LatestTranslationSummary},
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformKind {
    Macos,
    Windows,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlatformCapabilities {
    pub platform: PlatformKind,
    pub screen_capture: bool,
    pub window_capture: bool,
    pub system_ocr: bool,
    pub global_shortcuts: bool,
    pub credential_storage: bool,
}

impl PlatformCapabilities {
    pub fn current() -> Self {
        #[cfg(target_os = "macos")]
        return Self {
            platform: PlatformKind::Macos,
            screen_capture: true,
            window_capture: false,
            system_ocr: true,
            global_shortcuts: false,
            credential_storage: false,
        };

        #[cfg(target_os = "windows")]
        return Self {
            platform: PlatformKind::Windows,
            screen_capture: true,
            window_capture: false,
            system_ocr: false,
            global_shortcuts: false,
            credential_storage: false,
        };

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        Self {
            platform: PlatformKind::Unsupported,
            screen_capture: false,
            window_capture: false,
            system_ocr: false,
            global_shortcuts: false,
            credential_storage: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSourceKind {
    Display,
    Window,
    Application,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSourceSummary {
    pub source_id: String,
    pub catalog_revision: u64,
    pub kind: CaptureSourceKind,
    pub title: String,
    pub application_name: Option<String>,
    pub icon_data_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WindowSummary {
    pub label: String,
    pub visible: bool,
    pub always_on_top: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapSnapshot {
    pub platform: PlatformCapabilities,
    pub windows: Vec<WindowSummary>,
    pub settings: SettingsSummary,
    pub default_model: Option<ModelProfileSummary>,
    pub shortcuts: Vec<ShortcutBindingSummary>,
    pub capture: CaptureSettingsSummary,
    pub active_session: Option<ActiveSessionSummary>,
    pub latest_result: Option<LatestTranslationSummary>,
    pub history: HistorySummary,
    pub settings_revision: u64,
    pub capture_revision: u64,
    pub session_revision: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_source_contract_uses_platform_neutral_fields() {
        let source = CaptureSourceSummary {
            source_id: "display:1".into(),
            catalog_revision: 2,
            kind: CaptureSourceKind::Display,
            title: "主显示器".into(),
            application_name: None,
            icon_data_url: None,
        };
        let value = serde_json::to_value(source).expect("序列化捕获来源");
        assert_eq!(value["sourceId"], "display:1");
        assert!(value.get("exe").is_none());
    }
}
