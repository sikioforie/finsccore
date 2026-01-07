use shared::app::{AppState, SharedAppState};
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
pub fn scoring_set_config(name: &str) -> String {
    unimplemented!()
}

#[tauri::command]
pub fn scoring_calculate_score() {
    unimplemented!()
}

#[tauri::command]
pub fn scoring_share_score(id: &str) -> String {
    unimplemented!()
}

#[tauri::command]
pub fn scoring_get_scores() -> Vec<String> {
    unimplemented!()
}

/*----VERIFICATION----*/

#[tauri::command]
pub fn scoring_verify(id: &str) {
    unimplemented!()
}

#[tauri::command]
pub fn scoring_get_verifications() -> Vec<String> {
    unimplemented!()
}
