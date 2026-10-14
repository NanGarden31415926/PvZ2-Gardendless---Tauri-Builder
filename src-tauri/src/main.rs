// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_opener::OpenerExt;
use tauri::{Emitter, Manager};

use std::fs;
use std::path::PathBuf;

fn levels_dir(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = dirs::document_dir()
        .ok_or("no document_dir")?
        .join("PvZ2Gardendless")
        .join("levels");
    eprintln!("[levels_dir] base={:?}", base);
    fs::create_dir_all(&base).map_err(|e| format!("mkdir {:?}: {}", base, e))?;
    eprintln!("[levels_dir] ok");
    Ok(base)
}

#[tauri::command]
async fn save_level(app: tauri::AppHandle, path: String, content: String) -> Result<(), String> {
    let dir = levels_dir(&app)?;
    let file = dir.join(&path);
    eprintln!("[save_level] dir={:?}", dir);
    eprintln!("[save_level] file={:?}", file);

    if let Some(parent) = file.parent() {
        eprintln!("[save_level] parent={:?}", parent);
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {:?}: {}", parent, e))?;
    }

    let tmp = file.with_extension("tmp");
    eprintln!("[save_level] tmp={:?}", tmp);

    fs::write(&tmp, &content).map_err(|e| format!("write {:?}: {}", tmp, e))?;
    fs::rename(&tmp, &file).map_err(|e| format!("rename {:?} -> {:?}: {}", tmp, file, e))?;

    eprintln!("[save_level] done");
    Ok(())
}

#[tauri::command]
async fn load_level(app: tauri::AppHandle, path: String) -> Result<Option<String>, String> {
    let dir = levels_dir(&app)?;
    let file = dir.join(&path);
    if !file.exists() {
        return Ok(None);
    }
    fs::read_to_string(&file).map(Some).map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_level(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let dir = levels_dir(&app)?;
    let file = dir.join(&path);
    if file.exists() {
        fs::remove_file(&file).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn list_levels(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let dir = levels_dir(&app)?;
    let mut result = Vec::new();
    walk_dir(&dir, &dir, &mut result)?;
    Ok(result)
}

fn walk_dir(base: &PathBuf, dir: &PathBuf, out: &mut Vec<String>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_dir(base, &path, out)?;
        } else if path.is_file() {
            if let Ok(rel) = path.strip_prefix(base) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    Ok(())
}

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
            save_level,
            load_level,
            remove_level,
            list_levels,
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
