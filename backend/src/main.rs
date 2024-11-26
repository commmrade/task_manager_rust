use std::{sync::Arc};
mod handlers;
use axum::{
    routing::{get, post},
    Json, Router,
};
use handlers::{
    auth::{login, register}, database::{self, Db}, tasks::{get_tasks, post_tasks}
};


pub struct AppState {
    db : Arc<Db>
}


impl AppState {
    async fn new(url: String) -> Self {
        Self {
            db: Arc::new(Db::new(url).await.unwrap()),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("hello world");
    let app_state =
        Arc::new(AppState::new("mysql://klewy:root@localhost/task_manager".to_string()).await);
    let app: Router<()> = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/tasks", get(get_tasks))
        .route("/tasks", post(post_tasks))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


