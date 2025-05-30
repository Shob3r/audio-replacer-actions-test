mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::os_utils::get_username, 
            commands::project_file_utils::get_all_files, 
            commands::project_file_utils::get_subdirectories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
