use crate::desktop::models::RecoveryAction;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopErrorCode {
    PermissionDenied,
    PermissionRestartRequired,
    SourceStale,
    SourceNotFound,
    SourceUnavailable,
    InvalidRegion,
    CrossDisplayRegionUnsupported,
    ProtectedContent,
    UnsupportedCapability,
    BackendFailure,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopError {
    pub code: DesktopErrorCode,
    pub message: String,
    pub recoverable: bool,
    pub action: Option<RecoveryAction>,
}

impl DesktopError {
    pub fn source_stale() -> Self {
        Self {
            code: DesktopErrorCode::SourceStale,
            message: "捕获来源目录已刷新，请重新选择窗口。".to_string(),
            recoverable: true,
            action: Some(RecoveryAction::Retry),
        }
    }

    pub fn source_not_found() -> Self {
        Self {
            code: DesktopErrorCode::SourceNotFound,
            message: "未找到所选捕获来源，请刷新后重试。".to_string(),
            recoverable: true,
            action: Some(RecoveryAction::Retry),
        }
    }

    pub fn backend_failure() -> Self {
        Self {
            code: DesktopErrorCode::BackendFailure,
            message: "桌面能力暂时不可用，请稍后重试。".to_string(),
            recoverable: true,
            action: Some(RecoveryAction::Retry),
        }
    }
}

impl fmt::Display for DesktopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DesktopError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendErrorCode {
    PermissionDenied,
    PermissionRestartRequired,
    SourceUnavailable,
    UnsupportedCapability,
    Failure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendError {
    code: BackendErrorCode,
    diagnostic: String,
}

impl BackendError {
    pub fn new(code: BackendErrorCode, diagnostic: impl Into<String>) -> Self {
        Self {
            code,
            diagnostic: diagnostic.into(),
        }
    }

    pub fn failure(diagnostic: impl Into<String>) -> Self {
        Self::new(BackendErrorCode::Failure, diagnostic)
    }

    pub fn diagnostic(&self) -> &str {
        &self.diagnostic
    }

    pub fn to_public(&self) -> DesktopError {
        match self.code {
            BackendErrorCode::PermissionDenied => DesktopError {
                code: DesktopErrorCode::PermissionDenied,
                message: "未获得屏幕录制权限，请在系统设置中授权。".to_string(),
                recoverable: true,
                action: Some(RecoveryAction::OpenSystemSettings),
            },
            BackendErrorCode::PermissionRestartRequired => DesktopError {
                code: DesktopErrorCode::PermissionRestartRequired,
                message: "权限变更后需要重新启动 LoreLingo。".to_string(),
                recoverable: true,
                action: Some(RecoveryAction::RestartApplication),
            },
            BackendErrorCode::SourceUnavailable => DesktopError {
                code: DesktopErrorCode::SourceUnavailable,
                message: "捕获来源当前不可用，请刷新后重试。".to_string(),
                recoverable: true,
                action: Some(RecoveryAction::Retry),
            },
            BackendErrorCode::UnsupportedCapability => DesktopError {
                code: DesktopErrorCode::UnsupportedCapability,
                message: "当前平台不支持此桌面能力。".to_string(),
                recoverable: false,
                action: None,
            },
            BackendErrorCode::Failure => DesktopError::backend_failure(),
        }
    }
}
