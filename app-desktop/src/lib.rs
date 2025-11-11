/// A simple greet command.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// A simple save command.
#[tauri::command]
fn save_note(text: &str) -> Result<(), String> {
    // Get documents directory, create path '~/Documents/.static-tree-note'
    let dir = dirs::document_dir().ok_or("Could not find documents directory")?;
    let file_path = dir.join(".static-tree-note");

    // Save file
    std::fs::write(file_path, text).map_err(|e| e.to_string())?;

    Ok(())
}

/// A simple load command.
#[tauri::command]
fn load_note() -> Result<String, String> {
    // Get documents directory, create path '~/Documents/.static-tree-note'
    let dir = dirs::document_dir().ok_or("Could not find documents directory")?;
    let file_path = dir.join(".static-tree-note");

    // Load file, if not exist return empty string
    let content = std::fs::read_to_string(file_path).unwrap_or("".into());

    Ok(content)
}

/// The main entry point for the Tauri application (back end).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![
            greet,
            save_note,
            load_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
