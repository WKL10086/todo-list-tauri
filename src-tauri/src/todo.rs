use cuid2::create_id;

#[derive(serde::Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

pub trait Create {
    fn new(title: String) -> Self;
}

impl Create for Todo {
    fn new(title: String) -> Todo {
        let id = create_id();

        Todo {
            id: id,
            title: title,
            completed: false,
        }
    }
}
