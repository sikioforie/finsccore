pub mod openbank;
pub mod scoring;

use shared::app::{AppState, SharedAppState};
use tauri::State;

#[tauri::command]
pub fn get_state(state: State<'_, SharedAppState>) -> AppState {
    state.read().unwrap().clone()
}
