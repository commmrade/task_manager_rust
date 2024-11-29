use std::{sync::Arc};
mod handlers;
mod database_connector;
mod sessions;
use axum::{
    extract::Request, http::{HeaderValue, StatusCode}, middleware::from_fn, response::Response, routing::{get, post}, Json, Router
};
use database_connector::database::Db;
use handlers::{
    auth::{login, register}, tasks::{get_tasks, post_tasks}
};
use sessions::session_handler::check_token;



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



async fn auth_middleware(req : Request, next : axum::middleware::Next) -> Result<Response, StatusCode> {
    if check_token(req.headers().get("Authentication").map(|e| e.to_str().unwrap_or("")).unwrap_or("")) {
        return Ok(next.run(req).await)
    } else {
        println!("Token expired/incorrect");
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[tokio::main]
async fn main() {
    println!("hello world");
    let app_state =
        Arc::new(AppState::new("mysql://klewy:root@localhost/task_manager".to_string()).await);
    let app: Router<()> = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/tasks", get(get_tasks).layer(from_fn(auth_middleware)))
        .route("/tasks", post(post_tasks).layer(from_fn(auth_middleware)))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


