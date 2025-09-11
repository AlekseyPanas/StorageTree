// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;

fn main() {

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


