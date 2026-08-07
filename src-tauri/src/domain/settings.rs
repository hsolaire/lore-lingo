use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TranslationMode {
    Manual,
    Auto,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    Region,
    Window,
    App,
    Display,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelProfileSummary {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub model_name: String,
    pub api_base: Option<String>,
    pub has_secret: bool,
    pub options: serde_json::Value,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TranslationSettingsSummary {
    pub default_model_profile_id: Option<i64>,
    pub mode: TranslationMode,
    pub auto_interval_ms: u32,
    pub source_language: String,
    pub target_language: String,
    pub request_timeout_ms: u32,
    pub prefer_subtitles: bool,
    pub save_history: bool,
    pub history_retention_days: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OcrSettingsSummary {
    pub engine: String,
    pub api_base: Option<String>,
    pub has_secret: bool,
    pub recognition_languages: Vec<String>,
    pub sample_fps: f64,
    pub min_confidence: f64,
    pub deduplicate_frames: bool,
    pub options: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutBindingSummary {
    pub action: String,
    pub input_kind: String,
    pub binding_key: Option<String>,
    pub binding: Option<serde_json::Value>,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSettingsSummary {
    pub mode: CaptureMode,
    pub region: Option<CaptureRegionSummary>,
    pub app_selector: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CaptureRegionSummary {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSummary {
    pub translation: TranslationSettingsSummary,
    pub ocr: OcrSettingsSummary,
}
