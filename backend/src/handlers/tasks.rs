use std::{str::FromStr, sync::Arc};

use axum::{extract::{rejection::QueryRejection, Query, State}, http::{HeaderMap, StatusCode}, Json};
use serde::{Deserialize, Serialize};



use crate::AppState;

use super::session_handler::check_token_and_name;

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    username: String,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub text: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum TaskStatus {
    Completed,
    NotCompleted,
    InProgress,
}
impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Completed => "Completed".to_string(),
            Self::NotCompleted => "Not Completed".to_string(),
            Self::InProgress => "In Progress".to_string(),
        }
    }
}
impl FromStr for TaskStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Not Completed" => Ok(TaskStatus::NotCompleted),
            "In Progress" => Ok(TaskStatus::InProgress),
            "Completed" => Ok(TaskStatus::Completed),
            _ => Err("Not a valid enum".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub status: TaskStatus,
    pub comments: Vec<Comment>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub name: String,
    pub tasks: Vec<Task>,
}




pub async fn get_tasks(
    State(appstate): State<Arc<AppState>>,
    Query(result): Query<UserQuery>,
    headers: HeaderMap,
) -> Result<Json<Vec<Category>>, (StatusCode, String)> {
    if !result.username.is_empty() {
        if check_token_and_name(
            headers["Authentication"].to_str().unwrap_or(""),
            &result.username,
        ) {
            match appstate.db.fetch_tasks(result.username.clone()).await {
                Ok(vec) => return Ok(Json(vec)),
                Err(why) => {
                    println!("TAsk fetch error: {}", why);
                    return Err((StatusCode::BAD_REQUEST, "wrong data".into()));
                }
            }
        } else {
            println!("wrong token unauthorized");
            return Err((StatusCode::UNAUTHORIZED, "Wrong token".into()));
        }
    } else {
        return Err((StatusCode::BAD_REQUEST, "Wrong creds".into()));
    }
}
pub async fn post_tasks(
    State(appstate): State<Arc<AppState>>,
    query: Result<Query<UserQuery>, QueryRejection>,
    headers: HeaderMap,
    Json(result): Json<Vec<Category>>,
) -> Result<(), (StatusCode, String)> {
    if check_token_and_name(
        headers["Authentication"].to_str().unwrap_or(""),
        &query.as_ref().unwrap().username,
    ) {
        for element in result {
            match appstate
                .db
                .add_category(Query(query.as_ref().unwrap()).username.clone(), element)
                .await
            {
                Ok(()) => {}
                Err(why) => {
                    println!("Saving task error {}", why);
                }
            }
        }
        return Ok(());
    } else {
        println!("Token expired");
        return Err((StatusCode::UNAUTHORIZED, "Token error".into()));
    }
}