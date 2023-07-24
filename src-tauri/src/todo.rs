use cuid2::create_id;

#[derive(serde::Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        let id = create_id();

        Todo {
            id: id,
            title: title,
            completed: false,
        }
    }
}
