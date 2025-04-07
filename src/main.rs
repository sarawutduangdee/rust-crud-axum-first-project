mod routes;
mod handlers;
mod models;

use axum::{Router, serve};
use handlers::todo_handler::{AppState};
use routes::todo_routes::todo_routes;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };

    let app: Router = todo_routes(state);

    println!("🚀 Server running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
