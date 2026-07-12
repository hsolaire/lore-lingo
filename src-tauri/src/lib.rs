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
            OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
            IsWindowVisible,
        },
    },
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

    // Get exe path
    let exe = get_exe_name(pid).unwrap_or_else(|| "unknown.exe".to_string());

    let results = &*(lparam.0 as *const std::sync::Mutex<Vec<WindowInfo>>);
    if let Ok(mut list) = results.lock() {
        list.push(WindowInfo {
            id: hwnd.0 as u32,
            title,
            exe,
            icon_b64: None, // filled in Task 3
        });
    }

    TRUE
}

#[cfg(target_os = "windows")]
fn get_exe_name(pid: u32) -> Option<String> {
    use std::path::Path;
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = vec![0u16; 1024];
        let mut len = buf.len() as u32;
        QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len).ok()?;
        let _ = CloseHandle(handle);
        let path = String::from_utf16_lossy(&buf[..len as usize]);
        Some(Path::new(&path).file_name()?.to_string_lossy().to_string())
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
