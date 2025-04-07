use axum::{Router, routing::{get, put}};
use crate::handlers::todo_handler::{AppState, get_todos, create_todo, update_todo, delete_todo};

pub fn todo_routes(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", put(update_todo).delete(delete_todo))
        .with_state(state)
}
