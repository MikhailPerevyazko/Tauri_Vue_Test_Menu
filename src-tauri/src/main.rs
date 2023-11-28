#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Prevents additional console window on Windows in release, DO NOT REMOVE!!

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::env;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
fn main() {
    let args = Args::parse();
    let home_dir = std::env::var("HOME").unwrap();
    let file_path = match args.file_path {
        Some(path) => path,
        None => PathBuf::from(home_dir)
            .join(".config")
            .join("menuapp")
            .join("menu_config.yaml"),
    };
    tauri::Builder::default()
        .manage(PathState(file_path))
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_file, menu_open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(path: String, contents: String) {
    fs::write(path, contents).unwrap();
}
#[derive(Serialize, Deserialize, Debug)]
struct MenuItem {
    name: String,
    word: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Menu {
    menu: Vec<MenuItem>,
}

struct PathState(PathBuf);
impl PathState {
    fn path(&self) -> PathBuf {
        self.0.to_owned()
    }
}

#[tauri::command]
fn menu_open(state: tauri::State<PathState>) -> String {
    let file = std::fs::File::open(state.path()).unwrap();
    let json_menu: Menu = serde_yaml::from_reader(file).unwrap();
    let menu = serde_json::to_string_pretty(&json_menu).unwrap();
    menu
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: Option<PathBuf>,
}
