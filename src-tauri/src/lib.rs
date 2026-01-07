mod commands;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(shared::app::AppState::new());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::scoring::scoring_set_config,
            commands::scoring::scoring_calculate_score,
            commands::scoring::scoring_share_score,
            commands::scoring::scoring_get_scores,
            commands::scoring::scoring_verify,
            commands::scoring::scoring_get_verifications,
            commands::openbank::openbank_set_config,
            commands::get_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
