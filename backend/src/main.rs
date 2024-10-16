use std::sync::Arc;

use axum::{extract::{rejection::QueryRejection, Query}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct QUser {
    name: String,
    age: u32
}


struct AppState {
    
}
impl Default for AppState {
    fn default() -> Self {
        return Self{}
    }
}

#[tokio::main]
async fn main() {
    println!("hello world");

    let app_state = Arc::new(AppState::default());
    let app = Router::new()
    .route("/welcome", get(welcome))
    .route("/login", post(login))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


async fn welcome(result : Result<Query<QUser>, QueryRejection>) -> Result<(), (StatusCode, String)> {
    match result {
        Ok(Query(result)) => {
            todo!();
        }
        Err(_) => {
            return Err((StatusCode::BAD_GATEWAY, "Error".to_string()))
        }
    }
}

async fn login(result : Result<Query<QUser>, QueryRejection>) -> Result<(), (StatusCode, String)> {

    todo!("Implemnt test login klewy 1488");
}

