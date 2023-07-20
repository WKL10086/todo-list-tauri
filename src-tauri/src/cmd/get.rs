use tauri::command;

use crate::todo::Todo;

#[command]
pub fn get_todo_list() -> Vec<Todo> {
    let todo1 = Todo {
        id: 1,
        title: "Learn Tauri".to_string(),
        completed: false,
    };
    let todo2 = Todo {
        id: 2,
        title: "Learn Rust".to_string(),
        completed: false,
    };
    let todo3 = Todo {
        id: 3,
        title: "Learn Svelte".to_string(),
        completed: false,
    };
    let todo4 = Todo {
        id: 3,
        title: "Profit!".to_string(),
        completed: false,
    };
    vec![todo1, todo2, todo3, todo4]
}
