// Tauri 在 Windows release 模式下隐藏控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    lenslingo_lib::run();
}
