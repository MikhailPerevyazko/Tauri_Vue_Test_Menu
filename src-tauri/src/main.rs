#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Prevents additional console window on Windows in release, DO NOT REMOVE!!

use tauri::Manager;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::to_string;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_file, open_menu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(path: String, contents: String) {
    fs::write(path, contents).unwrap();
}
#[derive(Serialize, Deserialize)]
struct Menu {
    name: String,
    word: String,
}

fn open_menu() {
    let menu = Menu{};
    //    /home/Mikhail/.config/menuapp/menu_config.yaml'}

    let serialized = serde_yaml::to_string(&menu).unwrap();

    
}