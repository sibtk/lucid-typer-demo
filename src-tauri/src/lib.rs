mod engine;
mod commands;

use engine::core::new_shared_state;
use commands::typing::*;
use commands::device::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(new_shared_state())
        .invoke_handler(tauri::generate_handler![
            start_typing,
            stop_typing,
            pause_typing,
            resume_typing,
            get_engine_state,
            preview_typing,
            estimate_typing_time,
            get_device_fingerprint,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
