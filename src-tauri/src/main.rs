// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_opener::OpenerExt;
use tauri::{Emitter, Manager};
#[tauri::command]
fn center(_window: tauri::WebviewWindow) -> Result<(), String> {
    _window.center().map_err(|e| e.to_string())
}

#[tauri::command]
fn is_full_screen(_window: tauri::WebviewWindow) -> Result<bool, String> {
    _window.is_fullscreen().map_err(|e| e.to_string())
}

#[tauri::command]
fn full_screen(_window: tauri::WebviewWindow) -> Result<(), String> {
    _window.set_fullscreen(true).map_err(|e| e.to_string())
}

#[tauri::command]
fn window(_window: tauri::WebviewWindow) -> Result<(), String> {
    _window.set_fullscreen(false).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_size(_window: tauri::WebviewWindow, width: f64, height: f64) -> Result<(), String> {
    _window
        .set_size(tauri::LogicalSize::new(width, height))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn quit(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn open_dev_tools(_window: tauri::WebviewWindow) {
    // #[cfg(debug_assertions)]
    _window.open_devtools();
}

#[tauri::command]
fn close_dev_tools(_window: tauri::WebviewWindow) {
    // #[cfg(debug_assertions)]
    _window.close_devtools();
}

#[tauri::command]
fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_resolution(_window: tauri::WebviewWindow, _width: u32, _height: u32) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
fn nircmd(order: String) -> Result<(), String> {
    println!("nircmd: {}", order);
    Ok(())
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
       .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.emit(
                "sync-info",
                serde_json::json!({
                    "is_fullscreen": window.is_fullscreen().unwrap_or(false),
                    "massage": "tauri",
                    "resolutions": [],
                    "current_resolution": ""
                }),
            );

            let w = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(focused) = event {
                    if *focused {
                        let _ = w.emit("window-focused", true);
                    } else {
                        let _ = w.emit("window-unfocused", false);
                    }
                }
            });

            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .invoke_handler(tauri::generate_handler![
            center,
            is_full_screen,
            full_screen,
            window,
            set_size,
            quit,
            open_dev_tools,
            close_dev_tools,
            open_url,
            set_resolution,
            nircmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
