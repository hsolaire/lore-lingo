use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformKind {
    Macos,
    Windows,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSourceKind {
    Screen,
    Window,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturePermissionStatus {
    NotRequired,
    NotDetermined,
    Granted,
    Denied,
    RestartRequired,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAction {
    OpenSystemSettings,
    RestartApplication,
    Retry,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturePermission {
    pub status: CapturePermissionStatus,
    pub can_request: bool,
    pub action: Option<RecoveryAction>,
}

impl CapturePermission {
    pub fn new(
        status: CapturePermissionStatus,
        can_request: bool,
        action: Option<RecoveryAction>,
    ) -> Self {
        Self {
            status,
            can_request,
            action,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopCapabilities {
    pub platform: PlatformKind,
    pub architecture: String,
    pub screen_capture: bool,
    pub window_capture: bool,
    pub region_capture: bool,
    pub permission_prompt: bool,
    pub permission_restart_required: bool,
    pub self_exclusion: bool,
    pub system_ocr: bool,
    pub global_shortcuts: bool,
    pub credential_storage: bool,
}

impl DesktopCapabilities {
    pub fn new(platform: PlatformKind, architecture: impl Into<String>) -> Self {
        Self {
            platform,
            architecture: architecture.into(),
            screen_capture: false,
            window_capture: false,
            region_capture: false,
            permission_prompt: false,
            permission_restart_required: false,
            self_exclusion: false,
            system_ocr: false,
            global_shortcuts: false,
            credential_storage: false,
        }
    }

    pub fn with_screen_capture(mut self, supported: bool) -> Self {
        self.screen_capture = supported;
        self
    }

    pub fn with_window_capture(mut self, supported: bool) -> Self {
        self.window_capture = supported;
        self
    }

    pub fn with_region_capture(mut self, supported: bool) -> Self {
        self.region_capture = supported;
        self
    }

    pub fn with_permission_prompt(mut self, supported: bool) -> Self {
        self.permission_prompt = supported;
        self
    }

    pub fn with_permission_restart_required(mut self, required: bool) -> Self {
        self.permission_restart_required = required;
        self
    }

    pub fn with_self_exclusion(mut self, supported: bool) -> Self {
        self.self_exclusion = supported;
        self
    }

    pub fn with_system_ocr(mut self, supported: bool) -> Self {
        self.system_ocr = supported;
        self
    }

    pub fn with_global_shortcuts(mut self, supported: bool) -> Self {
        self.global_shortcuts = supported;
        self
    }

    pub fn with_credential_storage(mut self, supported: bool) -> Self {
        self.credential_storage = supported;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSummary {
    pub monitor_id: String,
    pub primary: bool,
    pub logical_size: LogicalSize,
    pub physical_size: PhysicalSize,
    pub scale_factor: f64,
    pub physical_origin: PhysicalPoint,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSourceSummary {
    pub source_id: String,
    pub catalog_revision: u64,
    pub kind: CaptureSourceKind,
    pub title: String,
    pub application_name: Option<String>,
    pub icon_data_url: Option<String>,
    pub monitor: Option<MonitorSummary>,
    pub capturable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSourceCatalog {
    pub catalog_revision: u64,
    pub sources: Vec<CaptureSourceSummary>,
}
