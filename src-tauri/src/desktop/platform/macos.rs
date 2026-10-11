use super::{discover_screens, should_include_window};
use crate::desktop::{
    BackendError, BackendErrorCode, CapturePermission, CapturePermissionStatus, CaptureSourceKind,
    DesktopCapabilities, DiscoveredCaptureSource, NativeCaptureSource, PlatformBackend,
    PlatformKind, RecoveryAction,
};
use core_foundation::base::{CFType, TCFType};
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::ConcreteCFType;
use core_graphics::access::ScreenCaptureAccess;
use core_graphics::window::{
    copy_window_info, kCGNullWindowID, kCGWindowIsOnscreen, kCGWindowLayer,
    kCGWindowListExcludeDesktopElements, kCGWindowListOptionOnScreenOnly, kCGWindowName,
    kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID, kCGWindowSharingState,
};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct MacosBackend {
    permission_requested: AtomicBool,
}

impl PlatformBackend for MacosBackend {
    fn capabilities(&self) -> DesktopCapabilities {
        DesktopCapabilities::new(PlatformKind::Macos, std::env::consts::ARCH)
            .with_screen_capture(true)
            .with_window_capture(super::macos_capture::available())
            .with_region_capture(true)
            .with_permission_prompt(true)
            .with_permission_restart_required(true)
            .with_self_exclusion(false)
            .with_system_ocr(true)
            .with_global_shortcuts(false)
            .with_credential_storage(true)
    }

    fn capture_permission(&self) -> Result<CapturePermission, BackendError> {
        if ScreenCaptureAccess.preflight() {
            return Ok(CapturePermission::new(
                CapturePermissionStatus::Granted,
                false,
                None,
            ));
        }
        if self.permission_requested.load(Ordering::Acquire) {
            return Ok(CapturePermission::new(
                CapturePermissionStatus::Denied,
                true,
                Some(RecoveryAction::OpenSystemSettings),
            ));
        }
        Ok(CapturePermission::new(
            CapturePermissionStatus::NotDetermined,
            true,
            None,
        ))
    }

    fn request_capture_permission(&self) -> Result<CapturePermission, BackendError> {
        self.permission_requested.store(true, Ordering::Release);
        if ScreenCaptureAccess.request() {
            Ok(CapturePermission::new(
                CapturePermissionStatus::Granted,
                false,
                None,
            ))
        } else {
            Ok(CapturePermission::new(
                CapturePermissionStatus::Denied,
                true,
                Some(RecoveryAction::OpenSystemSettings),
            ))
        }
    }

    fn discover_capture_sources(&self) -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
        if !ScreenCaptureAccess.preflight() {
            return Err(BackendError::new(
                BackendErrorCode::PermissionDenied,
                "macOS screen capture access is not granted",
            ));
        }

        let mut sources = discover_screens()?;
        sources.extend(discover_windows()?);
        Ok(sources)
    }
}

fn discover_windows() -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
    let options = kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;
    let window_info = copy_window_info(options, kCGNullWindowID)
        .ok_or_else(|| BackendError::failure("CGWindowListCopyWindowInfo returned null"))?;
    let self_pid = std::process::id();
    let mut sources = Vec::new();

    for raw_dictionary in window_info.get_all_values() {
        let dictionary = unsafe {
            <CFDictionary<CFString, CFType> as TCFType>::wrap_under_get_rule(
                raw_dictionary as CFDictionaryRef,
            )
        };
        let Some(window_id) = number_value(&dictionary, unsafe { kCGWindowNumber }) else {
            continue;
        };
        let owner_pid = number_value(&dictionary, unsafe { kCGWindowOwnerPID }).unwrap_or_default();
        let layer = number_value(&dictionary, unsafe { kCGWindowLayer }).unwrap_or_default() as i32;
        let visible = boolean_value(&dictionary, unsafe { kCGWindowIsOnscreen }).unwrap_or(true);
        let title = string_value(&dictionary, unsafe { kCGWindowName }).unwrap_or_default();

        if !should_include_window(&title, owner_pid as u32, self_pid, visible, layer) {
            continue;
        }

        let application_name = string_value(&dictionary, unsafe { kCGWindowOwnerName });
        let sharing_state =
            number_value(&dictionary, unsafe { kCGWindowSharingState }).unwrap_or_default();
        sources.push(DiscoveredCaptureSource::new(
            CaptureSourceKind::Window,
            title,
            application_name,
            None,
            sharing_state != 0 && super::macos_capture::available(),
            NativeCaptureSource::window(window_id as u64),
        ));
    }

    Ok(sources)
}

fn dictionary_value<T: ConcreteCFType>(
    dictionary: &CFDictionary<CFString, CFType>,
    key: CFStringRef,
) -> Option<T> {
    if key.is_null() {
        return None;
    }
    let key = unsafe { CFString::wrap_under_get_rule(key) };
    dictionary.find(&key)?.downcast::<T>()
}

fn string_value(dictionary: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<String> {
    dictionary_value::<CFString>(dictionary, key).map(|value| value.to_string())
}

fn number_value(dictionary: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<i64> {
    dictionary_value::<CFNumber>(dictionary, key)?.to_i64()
}

fn boolean_value(dictionary: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<bool> {
    dictionary_value::<CFBoolean>(dictionary, key).map(|value| value == CFBoolean::true_value())
}
