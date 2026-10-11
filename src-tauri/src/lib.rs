use base64::{engine::general_purpose, Engine as _};
use screenshots::Screen;
use std::sync::Arc;
use tauri::command;

pub mod commands;
pub mod db;
pub mod desktop;
pub mod domain;
pub mod services;

use commands::app::get_bootstrap_snapshot;
use commands::capture_translation::{
    cancel_capture_translation, capture_selector_payload, capture_window_translation,
    captured_region_preview, complete_capture_translation, get_capture_translation_config,
    get_captured_region, save_capture_translation_config, start_capture_translation,
    translate_captured_region, update_captured_region,
};
use commands::windows::{
    hide_window, list_app_windows, set_main_window_controls_visible, show_or_focus_window,
};
use desktop::{
    capture_permission_status, create_platform_backend, get_desktop_capabilities,
    list_capture_sources, request_capture_permission, DesktopService,
};
use services::bootstrap::BootstrapService;
use services::capture_translation::CaptureTranslationService;

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

    let image = screen.capture_area(x, y, w, h).map_err(|e| e.to_string())?;

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

/// 仅为 macOS 主窗口启用原生交通灯，保留其他窗口的无边框配置。
#[cfg(target_os = "macos")]
fn configure_main_window(config: &mut tauri::Config) {
    if let Some(main) = config
        .app
        .windows
        .iter_mut()
        .find(|window| window.label == "main")
    {
        main.decorations = true;
        main.title_bar_style = tauri::TitleBarStyle::Overlay;
        main.hidden_title = true;
        main.traffic_light_position =
            Some(tauri::utils::config::LogicalPosition { x: 14.0, y: 24.0 });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let capture_translation =
        CaptureTranslationService::new().expect("failed to initialize capture translation service");

    let context = tauri::generate_context!();
    #[cfg(target_os = "macos")]
    let context = {
        let mut context = context;
        configure_main_window(context.config_mut());
        context
    };

    tauri::Builder::default()
        .manage(Arc::new(capture_translation))
        .manage(Arc::new(DesktopService::new(create_platform_backend())))
        .setup(|app| {
            use tauri::Manager;

            let database = app
                .path()
                .app_data_dir()
                .ok()
                .map(|directory| db::connection::Database::initialize(&directory))
                .unwrap_or_else(|| {
                    Err(db::DatabaseError::InvalidData("应用数据目录不可用".into()))
                });
            let bootstrap = match &database {
                Ok(database) => BootstrapService::new(Ok(database.clone())),
                Err(_) => BootstrapService::unavailable(),
            };
            app.manage(Arc::new(
                services::translation_config::TranslationConfigService::new(database),
            ));
            app.manage(Arc::new(bootstrap));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_bootstrap_snapshot,
            get_desktop_capabilities,
            capture_permission_status,
            request_capture_permission,
            list_capture_sources,
            capture_region,
            list_screens,
            start_capture_translation,
            capture_window_translation,
            get_capture_translation_config,
            save_capture_translation_config,
            capture_selector_payload,
            get_captured_region,
            update_captured_region,
            captured_region_preview,
            translate_captured_region,
            complete_capture_translation,
            cancel_capture_translation,
            list_app_windows,
            show_or_focus_window,
            hide_window,
            set_main_window_controls_visible,
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[cfg(all(test, target_os = "macos"))]
mod window_presentation_tests {
    use super::configure_main_window;

    #[test]
    fn native_controls_only_apply_to_main_window() {
        let mut config: tauri::Config =
            serde_json::from_str(include_str!("../tauri.conf.json")).expect("读取窗口配置");
        let original = config.app.windows.clone();

        configure_main_window(&mut config);

        let main = config
            .app
            .windows
            .iter()
            .find(|window| window.label == "main")
            .expect("主窗口存在");
        assert!(main.decorations);
        assert!(main.hidden_title);
        assert!(matches!(
            main.title_bar_style,
            tauri::TitleBarStyle::Overlay
        ));
        assert!(main.traffic_light_position.is_some());
        for (before, after) in original.iter().zip(&config.app.windows) {
            if before.label != "main" {
                assert_eq!(
                    serde_json::to_value(before).expect("序列化原配置"),
                    serde_json::to_value(after).expect("序列化当前配置")
                );
            }
        }
    }
}
