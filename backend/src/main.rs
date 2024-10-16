use std::sync::Arc;

use axum::{extract::{rejection::QueryRejection, Query}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AuthData {
    name: String,
    password: String
}
struct Resp {
    token: String
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
    .route("/login", get(login))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

//To auth module
async fn login(result : Result<Query<AuthData>, QueryRejection>) -> Result<(), (StatusCode, String)> {

    match result {
        Ok(Query(result)) => {
            println!("{} {}", result.name, result.password);
            if result.name == "klewy" && result.password == "1488" {
                return Ok(())
            } else {
                return Err((StatusCode::BAD_REQUEST, "Wrong login/password".to_string()))
            }
        }
        Err(_) => {
            println!("Error, wrong data");
            return Err((StatusCode::BAD_REQUEST, "".to_string()));
        }
    }
}
async fn register(result : Result<Query<AuthData>, QueryRejection>) -> Result<(), (StatusCode, String)> {
    todo!()
}

