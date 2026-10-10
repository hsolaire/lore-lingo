use std::{sync::mpsc, time::Duration};

use objc2::{msg_send, runtime::AnyObject};
use tauri::WebviewWindow;

use crate::domain::translation::PublicError;

pub fn set_controls_visible(window: WebviewWindow, visible: bool) -> Result<(), PublicError> {
    // 步骤 1. 将 AppKit 操作派发到主线程，只把窗口所有者和显示状态带入闭包。
    let (sender, receiver) = mpsc::sync_channel(1);
    let main_window = window.clone();
    window
        .run_on_main_thread(move || {
            let result = main_window
                .ns_window()
                .map_err(|_| PublicError::window_unavailable())
                .and_then(|pointer| {
                    // 步骤 2. 窗口持有期间读取系统按钮；原生对象不越过线程或 IPC 边界。
                    // 安全性：此闭包运行于 AppKit 主线程，main_window 保证 NSWindow 的生命周期。
                    unsafe {
                        let window = (pointer as *const AnyObject)
                            .as_ref()
                            .ok_or_else(PublicError::window_unavailable)?;
                        for button_type in [0usize, 1, 2] {
                            let button: *mut AnyObject =
                                msg_send![window, standardWindowButton: button_type];
                            let button = button
                                .as_ref()
                                .ok_or_else(PublicError::window_unavailable)?;
                            let _: () = msg_send![button, setHidden: !visible];
                        }
                    }
                    Ok(())
                });
            let _ = sender.send(result);
        })
        .map_err(|_| PublicError::window_unavailable())?;

    // 步骤 3. 在调用者的后台线程等待有限时间，失败时返回结构化错误而不是假装按钮已更新。
    receiver
        .recv_timeout(Duration::from_secs(2))
        .map_err(|_| PublicError::window_unavailable())?
}
