// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::thread::sleep;
use std::time::Duration;
use tauri::Manager;
use tauri::PhysicalPosition;
use tauri::Theme;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_positioner::{Position, WindowExt};
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(desktop)]
            let win = app.get_webview_window("main").unwrap();
            win.hide().unwrap();

            let _ = app.handle().plugin(tauri_plugin_positioner::init());
            let _ = app.handle().plugin(tauri_plugin_shell::init());

            let _ = win.as_ref().window().move_window(Position::TopRight);
            let current_position = win.outer_position().unwrap();
            let offset_position = PhysicalPosition {
                x: current_position.x - 24,
                y: current_position.y + 24,
            };
            let _ = win.set_position(tauri::Position::Physical(offset_position));

            tauri::async_runtime::spawn(async move {
                // adapt sleeping time to be long enough
                sleep(Duration::from_millis(10));
                win.show().unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
