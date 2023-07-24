use crate::todo::Todo;
use tauri::command;

#[command]
pub fn get_init_todo_list() -> Vec<Todo> {
    let todo1 = Todo::new("Learn Tauri".to_string());

    let todo2 = Todo::new("Learn Rust".to_string());

    let todo3 = Todo::new("Learn Svelte".to_string());

    let todo4 = Todo::new("Profit!".to_string());

    vec![todo1, todo2, todo3, todo4]
}
