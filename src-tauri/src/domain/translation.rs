use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LiveSessionStatus {
    Idle,
    Starting,
    Running,
    Paused,
    Stopping,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayoutTaskStatus {
    Idle,
    Selecting,
    Capturing,
    Recognizing,
    Translating,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveSessionSummary {
    pub session_id: i64,
    pub mode: String,
    pub status: LiveSessionStatus,
    pub started_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LatestTranslationSummary {
    pub history_id: i64,
    pub source_text: String,
    pub translated_text: Option<String>,
    pub source_language: String,
    pub target_language: String,
    pub result_source: String,
    pub status: String,
    pub created_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicError {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl PublicError {
    pub fn database_unavailable() -> Self {
        Self {
            code: "database_error".into(),
            message: "本地数据库暂时不可用，请重启应用后重试。".into(),
            recoverable: true,
            action: Some("restart_app".into()),
        }
    }

    pub fn unsupported_platform() -> Self {
        Self {
            code: "unsupported".into(),
            message: "当前平台尚未提供完整的桌面能力。".into(),
            recoverable: false,
            action: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_error_omits_empty_action() {
        let value = serde_json::to_value(PublicError::unsupported_platform()).expect("序列化错误");
        assert!(value.get("action").is_none());
    }

    #[test]
    fn unknown_live_status_is_rejected() {
        let value = serde_json::from_str::<LiveSessionStatus>("\"recovering\"");
        assert!(value.is_err());
    }
}
