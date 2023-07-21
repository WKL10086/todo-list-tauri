use crate::todo::{Create, Todo};
use tauri::command;

#[command]
pub fn get_init_todo_list() -> Vec<Todo> {
    let todo1: Todo = Create::new("Learn Tauri".to_string());

    let todo2: Todo = Create::new("Learn Rust".to_string());

    let todo3: Todo = Create::new("Learn Svelte".to_string());

    let todo4: Todo = Create::new("Profit!".to_string());

    vec![todo1, todo2, todo3, todo4]
}
