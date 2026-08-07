use super::{discover_screens, should_include_window};
use crate::desktop::{
    BackendError, CapturePermission, CapturePermissionStatus, CaptureSourceKind,
    DesktopCapabilities, DiscoveredCaptureSource, NativeCaptureSource, PlatformBackend,
    PlatformKind,
};
use base64::{engine::general_purpose, Engine as _};
use std::path::Path;
use std::sync::Mutex;
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{CloseHandle, BOOL, HWND, LPARAM, TRUE},
        Graphics::Gdi::{
            CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
            ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        },
        System::Threading::{
            GetCurrentProcessId, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::{
            Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON},
            WindowsAndMessaging::{
                DestroyIcon, DrawIconEx, EnumWindows, GetWindowTextLengthW, GetWindowTextW,
                GetWindowThreadProcessId, IsWindowVisible, DI_NORMAL,
            },
        },
    },
};

pub struct WindowsBackend;

impl PlatformBackend for WindowsBackend {
    fn capabilities(&self) -> DesktopCapabilities {
        DesktopCapabilities::new(PlatformKind::Windows, std::env::consts::ARCH)
            .with_screen_capture(true)
            .with_window_capture(false)
            .with_region_capture(true)
            .with_permission_prompt(false)
            .with_permission_restart_required(false)
            .with_self_exclusion(false)
            .with_system_ocr(false)
            .with_global_shortcuts(false)
            .with_credential_storage(false)
    }

    fn capture_permission(&self) -> Result<CapturePermission, BackendError> {
        Ok(CapturePermission::new(
            CapturePermissionStatus::NotRequired,
            false,
            None,
        ))
    }

    fn request_capture_permission(&self) -> Result<CapturePermission, BackendError> {
        self.capture_permission()
    }

    fn discover_capture_sources(&self) -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
        let mut sources = discover_screens()?;
        sources.extend(discover_windows()?);
        Ok(sources)
    }
}

fn discover_windows() -> Result<Vec<DiscoveredCaptureSource>, BackendError> {
    let results = Mutex::new(Vec::new());
    let results_ptr = &results as *const Mutex<Vec<DiscoveredCaptureSource>> as isize;

    unsafe {
        EnumWindows(Some(enum_windows_callback), LPARAM(results_ptr))
            .map_err(|error| BackendError::failure(format!("EnumWindows failed: {error}")))?;
    }

    results
        .into_inner()
        .map_err(|_| BackendError::failure("window discovery lock was poisoned"))
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let visible = IsWindowVisible(hwnd).as_bool();
    let title_len = GetWindowTextLengthW(hwnd);
    if title_len == 0 {
        return TRUE;
    }

    let mut title_buffer = vec![0_u16; (title_len + 1) as usize];
    let copied = GetWindowTextW(hwnd, &mut title_buffer);
    let title = String::from_utf16_lossy(&title_buffer[..copied as usize]);
    let mut process_id = 0_u32;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));

    if !should_include_window(&title, process_id, GetCurrentProcessId(), visible, 0) {
        return TRUE;
    }

    let (executable_path, application_name) = get_executable_info(process_id)
        .map(|(path, name)| (Some(path), Some(name)))
        .unwrap_or((None, None));
    let icon_data_url = executable_path
        .as_deref()
        .and_then(get_executable_icon_data_url);
    let results = &*(lparam.0 as *const Mutex<Vec<DiscoveredCaptureSource>>);

    if let Ok(mut sources) = results.lock() {
        sources.push(DiscoveredCaptureSource::new(
            CaptureSourceKind::Window,
            title,
            application_name,
            icon_data_url,
            true,
            NativeCaptureSource::window(hwnd.0 as u64),
        ));
    }

    TRUE
}

fn get_executable_info(process_id: u32) -> Option<(String, String)> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id).ok()?;
        let mut buffer = vec![0_u16; 1024];
        let mut length = buffer.len() as u32;
        let result = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut length,
        );
        let _ = CloseHandle(handle);
        result.ok()?;
        let full_path = String::from_utf16_lossy(&buffer[..length as usize]);
        let basename = Path::new(&full_path)
            .file_name()?
            .to_string_lossy()
            .to_string();
        Some((full_path, basename))
    }
}

fn get_executable_icon_data_url(executable_path: &str) -> Option<String> {
    use image::{ImageBuffer, RgbaImage};
    use std::io::Cursor;

    unsafe {
        let wide_path: Vec<u16> = executable_path
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut shell_info: SHFILEINFOW = std::mem::zeroed();
        let result = SHGetFileInfoW(
            PCWSTR(wide_path.as_ptr()),
            Default::default(),
            Some(&mut shell_info),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );
        if result == 0 || shell_info.hIcon.is_invalid() {
            return None;
        }

        let mut bitmap_info: BITMAPINFO = std::mem::zeroed();
        bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_info.bmiHeader.biWidth = 32;
        bitmap_info.bmiHeader.biHeight = -32;
        bitmap_info.bmiHeader.biPlanes = 1;
        bitmap_info.bmiHeader.biBitCount = 32;
        bitmap_info.bmiHeader.biCompression = BI_RGB.0;

        let screen_dc = GetDC(HWND::default());
        let bitmap = CreateCompatibleBitmap(screen_dc, 32, 32);
        ReleaseDC(HWND::default(), screen_dc);
        let drawing_dc = CreateCompatibleDC(None);
        let mut pixels = vec![0_u8; 32 * 32 * 4];
        let previous_object = SelectObject(drawing_dc, bitmap);
        let draw_result = DrawIconEx(
            drawing_dc,
            0,
            0,
            shell_info.hIcon,
            32,
            32,
            0,
            None,
            DI_NORMAL,
        );
        if draw_result.is_ok() {
            GetDIBits(
                drawing_dc,
                bitmap,
                0,
                32,
                Some(pixels.as_mut_ptr().cast()),
                &mut bitmap_info,
                DIB_RGB_COLORS,
            );
        }
        SelectObject(drawing_dc, previous_object);
        let _ = DeleteObject(bitmap);
        let _ = DeleteDC(drawing_dc);
        let _ = DestroyIcon(shell_info.hIcon);
        draw_result.ok()?;

        for pixel in pixels.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        let image: RgbaImage = ImageBuffer::from_raw(32, 32, pixels)?;
        let mut buffer = Cursor::new(Vec::new());
        image::DynamicImage::from(image)
            .write_to(&mut buffer, image::ImageFormat::Png)
            .ok()?;
        Some(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(buffer.into_inner())
        ))
    }
}
