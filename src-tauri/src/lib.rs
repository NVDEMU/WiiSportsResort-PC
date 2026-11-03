mod commands;
mod models;

use commands::ProcessState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(ProcessState::default())
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::inspect_dolphin,
            commands::inspect_game,
            commands::launch_game,
            commands::launch_dolphin_ui,
            commands::poll_launch_status,
            commands::import_mod_archive,
            commands::set_mod_enabled,
            commands::remove_mod,
            commands::save_gecko_codes,
            commands::backup_profile_data,
            commands::list_backups,
            commands::restore_backup,
            commands::get_profile_data_path,
            commands::open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running Wii Sports Resort Enhanced");
}
