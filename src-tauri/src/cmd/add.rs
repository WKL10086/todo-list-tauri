use crate::todo::Todo;
use tauri::command;

#[command]
pub fn create_todo(title: String) -> Todo {
    Todo::new(title)
}
