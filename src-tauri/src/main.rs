// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod todo;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::get_init_todo_list,
            cmd::create_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
