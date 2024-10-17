use std::{result, sync::Arc};

use axum::{extract::{rejection::QueryRejection, Query}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AuthData {
    name: String,
    password: String
}

#[derive(Deserialize, Serialize)]
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
    .route("/register", post(register))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

//To auth module todo: 
async fn login(result : Result<Query<AuthData>, QueryRejection>) -> Result<Json<Resp>, (StatusCode, String)> {

    match result {
        Ok(Query(result)) => {
            println!("{} {}", result.name, result.password);
            if true && (!result.name.is_empty() && !result.password.is_empty()) {
                Ok(Json(Resp { token: "".to_string() }))
            } else {
                Err((StatusCode::BAD_REQUEST, "Wrong credentials".into()))
            }
        }
        Err(_) => {
            println!("Error, wrong data");
            Err((StatusCode::BAD_REQUEST, "".to_string()))
        }
    }
}
async fn register(result : Result<Query<AuthData>, QueryRejection>) -> Result<Json<Resp>, (StatusCode, String)> {
    match result {
        Ok(Query(result)) => {
            println!("{}, {}", result.name, result.password);
            if true && (!result.name.is_empty() && !result.password.is_empty()) {
                Ok(Json(Resp { token: "".to_string() }))
            } else {
                Err((StatusCode::BAD_REQUEST, "Wrong credentials".into()))
            }
        }
        Err(_) => {
            println!("Incorrect data");
            Err((StatusCode::BAD_REQUEST, "".to_string()))
        }
    }
}

