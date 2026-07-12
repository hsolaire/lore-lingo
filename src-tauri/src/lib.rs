use base64::{engine::general_purpose, Engine as _};
use screenshots::Screen;
use tauri::command;
use tauri::Manager;
use serde::Serialize;

#[cfg(target_os = "windows")]
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{CloseHandle, BOOL, HWND, LPARAM},
        System::Threading::{
            GetCurrentProcessId, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
            IsWindowVisible,
        },
    },
};

#[cfg(target_os = "windows")]
use windows::Win32::{
    UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON},
    Graphics::Gdi::{
        CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
        ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    },
    UI::WindowsAndMessaging::{DestroyIcon, DrawIconEx, DI_NORMAL},
};

#[derive(Serialize)]
struct WindowInfo {
    id: u32,
    title: String,
    exe: String,
    icon_b64: Option<String>,
}

/// 截取指定屏幕区域，返回 PNG base64 字符串。
/// x/y/w/h 均为逻辑像素（Tauri WebviewWindow 坐标系）。
#[command]
fn capture_region(x: i32, y: i32, w: u32, h: u32) -> Result<String, String> {
    let screens = Screen::all().map_err(|e| e.to_string())?;
    // 取包含目标点的第一块屏幕
    let screen = screens
        .into_iter()
        .find(|s| {
            let info = &s.display_info;
            x >= info.x
                && x < info.x + info.width as i32
                && y >= info.y
                && y < info.y + info.height as i32
        })
        .ok_or_else(|| "未找到对应屏幕".to_string())?;

    let image = screen
        .capture_area(x, y, w, h)
        .map_err(|e| e.to_string())?;

    // screenshots 0.8 返回 image::RgbaImage，用 Cursor + PNG 编码器编码
    let mut buf = std::io::Cursor::new(Vec::new());
    image
        .write_to(&mut buf, screenshots::image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    Ok(general_purpose::STANDARD.encode(buf.into_inner()))
}

/// 列出所有屏幕的基本信息。
#[command]
fn list_screens() -> Vec<serde_json::Value> {
    Screen::all()
        .unwrap_or_default()
        .iter()
        .map(|s| {
            let i = &s.display_info;
            serde_json::json!({
                "id": i.id,
                "x": i.x,
                "y": i.y,
                "width": i.width,
                "height": i.height,
                "scaleFactor": i.scale_factor,
                "isPrimary": i.is_primary,
            })
        })
        .collect()
}

#[command]
fn list_windows() -> Vec<WindowInfo> {
    #[cfg(not(target_os = "windows"))]
    return vec![];

    #[cfg(target_os = "windows")]
    {
        let results: std::sync::Mutex<Vec<WindowInfo>> = std::sync::Mutex::new(Vec::new());
        let results_ptr = &results as *const _ as isize;

        unsafe {
            let _ = EnumWindows(
                Some(enum_windows_callback),
                LPARAM(results_ptr),
            );
        }

        results.into_inner().unwrap_or_default()
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    use windows::Win32::Foundation::TRUE;

    // Skip invisible windows
    if !IsWindowVisible(hwnd).as_bool() {
        return TRUE;
    }

    // Skip windows with no title
    let title_len = GetWindowTextLengthW(hwnd);
    if title_len == 0 {
        return TRUE;
    }

    let mut title_buf = vec![0u16; (title_len + 1) as usize];
    GetWindowTextW(hwnd, &mut title_buf);
    let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

    // Get process id
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));

    // 跳过 LensLingo 自身的窗口（main + overlay）
    if pid == GetCurrentProcessId() {
        return TRUE;
    }

    // Get exe path and name
    let (exe_path, exe) = get_exe_info(pid)
        .unwrap_or_else(|| ("".to_string(), "unknown.exe".to_string()));
    let icon_b64 = if exe_path.is_empty() { None } else { get_exe_icon_b64(&exe_path) };

    let results = &*(lparam.0 as *const std::sync::Mutex<Vec<WindowInfo>>);
    if let Ok(mut list) = results.lock() {
        list.push(WindowInfo {
            // HWND values on 64-bit Windows always fit in 32 bits (Win32 subsystem guarantee)
            id: hwnd.0 as u32,
            title,
            exe,
            icon_b64,
        });
    }

    TRUE
}

#[cfg(target_os = "windows")]
fn get_exe_info(pid: u32) -> Option<(String, String)> {
    // Returns (full_path, basename)
    use std::path::Path;
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = vec![0u16; 1024];
        let mut len = buf.len() as u32;
        let query_result = QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len);
        let _ = CloseHandle(handle);
        query_result.ok()?;
        let full_path = String::from_utf16_lossy(&buf[..len as usize]);
        let basename = Path::new(&full_path).file_name()?.to_string_lossy().to_string();
        Some((full_path, basename))
    }
}

#[cfg(target_os = "windows")]
fn get_exe_icon_b64(exe_path: &str) -> Option<String> {
    use std::io::Cursor;
    use image::{ImageBuffer, RgbaImage};

    unsafe {
        let wide: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut shfi: SHFILEINFOW = std::mem::zeroed();
        let ret = SHGetFileInfoW(
            windows::core::PCWSTR(wide.as_ptr()),
            Default::default(),
            Some(&mut shfi),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );
        if ret == 0 || shfi.hIcon.is_invalid() {
            return None;
        }

        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = 32;
        bmi.bmiHeader.biHeight = -32; // top-down
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB.0;

        let screen_dc = GetDC(HWND::default());
        let hbmp = CreateCompatibleBitmap(screen_dc, 32, 32);
        ReleaseDC(HWND::default(), screen_dc);
        let dc = CreateCompatibleDC(None);
        let mut pixels: Vec<u8> = vec![0u8; 32 * 32 * 4];
        let old = SelectObject(dc, hbmp);
        let draw_result = DrawIconEx(dc, 0, 0, shfi.hIcon, 32, 32, 0, None, DI_NORMAL);
        if draw_result.is_ok() {
            GetDIBits(dc, hbmp, 0, 32, Some(pixels.as_mut_ptr() as *mut _), &mut bmi, DIB_RGB_COLORS);
        }
        SelectObject(dc, old);
        let _ = DeleteObject(hbmp);
        let _ = DeleteDC(dc);
        let _ = DestroyIcon(shfi.hIcon);
        draw_result.ok()?;

        // Windows DIB is BGRA — convert to RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // B <-> R
        }

        let img: RgbaImage = ImageBuffer::from_raw(32, 32, pixels)?;
        let dyn_img = image::DynamicImage::from(img);
        let mut buf = Cursor::new(Vec::new());
        dyn_img.write_to(&mut buf, image::ImageFormat::Png).ok()?;
        Some(base64::engine::general_purpose::STANDARD.encode(buf.into_inner()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![capture_region, list_screens, list_windows])
        .setup(|app| {
            #[cfg(debug_assertions)]
            if let Some(webview) = app.get_webview_window("main") {
                webview.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
