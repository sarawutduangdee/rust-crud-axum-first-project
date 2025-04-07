use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}
