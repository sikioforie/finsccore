use shared::app::{AppState, SharedAppState};
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
pub fn openbank_set_config() {
    unimplemented!();
}

// #[tauri::command]
pub fn openbank_get_transactions() -> Vec<String> {
    unimplemented!()
}
