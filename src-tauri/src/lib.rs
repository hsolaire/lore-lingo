use base64::{engine::general_purpose, Engine as _};
use screenshots::Screen;
use tauri::command;
use tauri::Manager;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![capture_region, list_screens])
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
