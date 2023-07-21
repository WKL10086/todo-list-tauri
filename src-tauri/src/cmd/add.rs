use crate::todo::{Create, Todo};
use tauri::command;

#[command]
pub fn create_todo(title: String) -> Todo {
    Create::new(title)
}
