use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

/// A simple greet command.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// A simple save command. This is a temporary endpoint to manage a
/// static file.
#[tauri::command]
fn save_note(app: AppHandle<Wry>, text: &str) -> Result<(), String> {
    app.store("test.json")
        .map_err(|e| format!("Failed to open store: {}", e))?
        .set("static", text);
    Ok(())
}

/// A simple load command. This is a temporary endpoint to manage a
/// static file.
#[tauri::command]
fn load_note(app: AppHandle<Wry>) -> Result<String, String> {
    let content = app
        .store("test.json")
        .map_err(|e| format!("Failed to open store: {}", e))?
        .get("static")
        .map(|v| {
            v.as_str()
                .unwrap_or("<b>Edit this to save</b> the static file! <b>Do not mess up the JSON file this time.</b>")
                .to_string()
        })
        .unwrap_or("<b>Edit this to save</b> the static file!".to_string());
    Ok(content)
}

/// The main entry point for the Tauri application (back end).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, save_note, load_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
