use tauri::command;

use crate::todo::Todo;

#[command]
pub fn get_todo_list() -> Vec<Todo> {
    let todo1 = Todo {
        id: "".into(),
        title: "Learn Tauri".to_string(),
        completed: false,
    };

    vec![todo1]
}
