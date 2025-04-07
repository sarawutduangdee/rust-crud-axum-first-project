use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use std::sync::{Arc, Mutex};

use crate::models::todo::{Todo, CreateTodo};

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

pub async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap().clone();
    Json(todos)
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: payload.title,
    };
    state.todos.lock().unwrap().push(todo.clone());
    Json(todo)
}

pub async fn update_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let mut todos = state.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.title = payload.title;
        return Json(todo.clone());
    }
    Json(Todo {
        id: id.clone(),
        title: "Not Found".to_string(),
    })
}

pub async fn delete_todo(Path(id): Path<String>, State(state): State<AppState>) -> Json<&'static str> {
    let mut todos = state.todos.lock().unwrap();
    todos.retain(|t| t.id != id);
    Json("Deleted")
}
