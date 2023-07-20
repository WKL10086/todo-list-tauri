#[derive(serde::Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

// #[derive(serde::Serialize)]
// pub struct TodoList {
//     pub todo_list: Vec<Todo>,
// }

// trait Create {
//     fn create(&self, title: String) -> Todo;
// }
