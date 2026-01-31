pub mod services;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_pool = tauri::async_runtime::block_on(services::db::init_db());
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db_pool)
        .invoke_handler(tauri::generate_handler![
            
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
