// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn hide_window(webview_window: tauri::WebviewWindow) {
    println!("WebviewWindow: {}", webview_window.label());
    webview_window.hide().unwrap();
}

use tauri::Manager;
use tauri::PhysicalPosition;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_positioner::{Position, WindowExt};
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![hide_window])
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

            let _ = win.show().unwrap();

            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_shift_c_shortcut =
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyC);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            println!("{:?}", shortcut);
                            if shortcut == &ctrl_shift_c_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        //if the webview is hidden, show it and vice versa
                                        if win.is_visible().unwrap() {
                                            win.hide().unwrap();
                                        } else {
                                            win.show().unwrap();
                                        }
                                    }
                                    ShortcutState::Released => {
                                        // nothing to do
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_shift_c_shortcut)?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
