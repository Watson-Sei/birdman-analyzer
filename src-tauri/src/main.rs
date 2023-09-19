// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu,AboutMetadata};



fn main() {
    let mainmenu = Submenu::new(
        "BirdmanAnalyzer",
        Menu::new()
            .add_native_item(MenuItem::About(
                "BirdmanAnalyzer".to_string(),
                AboutMetadata::default(),
            ))
    );
    let filemenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("open".to_string(), "Open"))
    );
    let menu = Menu::new()
        .add_submenu(mainmenu)
        .add_submenu(filemenu);
    tauri::Builder::default()
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
