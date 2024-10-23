use std::{result, sync::Arc};

mod handlers;
use axum::{extract::{rejection::QueryRejection, Query, State}, http::StatusCode, routing::{get, post}, Json, Router};
use handlers::database::{self, Db};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;




#[derive(Deserialize, Serialize)]
struct LoginData {
    name: String,
    password: String
}

#[derive(Deserialize, Serialize)]
struct RegData {
    name: String,
    password: String,
    email : String
}

#[derive(Deserialize, Serialize)]
struct Resp {
    token: String
}

struct AppState {
   db : Arc<Db>
}
impl AppState {
    async fn new(url : String) -> Self {
        Self { db: Arc::new(Db::new(url).await.unwrap()) }
    }
}

#[tokio::main]
async fn main() {
    println!("hello world");
    let app_state = Arc::new(AppState::new("mysql://klewy:root@localhost/task_manager".to_string()).await);
    let app : Router<()> = Router::new()
    .route("/login", get(login))
    .route("/register", post(register))
    .with_state(app_state);
 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}


async fn login(State(appstate) : State<Arc<AppState>>, result : Result<Query<LoginData>, QueryRejection>) -> Result<Json<Resp>, (StatusCode, String)> {

    match result {
        Ok(Query(result)) => {
        
            match appstate.db.login_user(result.name, result.password).await {
                Ok(()) => {
                    println!("Successful login");
                    return Ok(Json(Resp { token: "1488".to_string() }))
                }
                Err(why) => {
                    println!("Login error: {}", why);
                    return Err((StatusCode::BAD_REQUEST, "".to_string()))
                }
            }
        }
        Err(_) => {
            println!("Error, wrong data");
            Err((StatusCode::NO_CONTENT, "".to_string()))
        }
    }
}
async fn register(State(appstate) : State<Arc<AppState>>, result : Result<Query<RegData>, QueryRejection>) -> Result<Json<Resp>, (StatusCode, String)> {
    match result {
        Ok(Query(result)) => {
            if !result.name.is_empty() && !result.password.is_empty() {
                
                match appstate.db.add_user(result.name, result.password, result.email).await {
                    Ok(()) => {
                       
                        Ok(Json(Resp { token: "1488".to_string() }))
                    }
                    Err(why) => {
                        println!("Register error: {}", why);
                        return Err((StatusCode::BAD_REQUEST, "Wrong credentials".into()))
                    }
                }
                
            } else {
                Err((StatusCode::BAD_REQUEST, "Wrong credentials".into()))
            }
        }
        Err(_) => {
            println!("Incorrect data");
            Err((StatusCode::NO_CONTENT, "".to_string()))
        }
    }
}

