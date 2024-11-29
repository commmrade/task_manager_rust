use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{sessions::session_handler::make_token, AppState};



#[derive(Deserialize, Serialize)]
pub struct LoginData {
    name: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegData {
    name: String,
    password: String,
    email: String,
}



pub async fn login(
    State(appstate): State<Arc<AppState>>,
    Json(result): Json<LoginData>,
) -> Result<String, (StatusCode, String)> {
    match appstate
        .db
        .login_user(result.name.clone(), result.password)
        .await
    {
        Ok(()) => {
            println!("Successful login");
            return Ok(make_token(&result.name));
        }
        Err(why) => {
            println!("Login error: {}", why);
            return Err((StatusCode::BAD_REQUEST, "".to_string()));
        }
    }
}


pub async fn register(
    State(appstate): State<Arc<AppState>>,
    Json(result): Json<RegData>,
) -> Result<String, (StatusCode, String)> {
    if !result.name.is_empty() && !result.password.is_empty() {
        match appstate
            .db
            .add_user(result.name.clone(), result.password, result.email)
            .await
        {
            Ok(()) => Ok(make_token(&result.name)),
            Err(why) => {
                println!("Register error: {}", why);
                return Err((StatusCode::BAD_REQUEST, "Wrong credentials".into()));
            }
        }
    } else {
        Err((StatusCode::BAD_REQUEST, "Wrong credentials".into()))
    }
}