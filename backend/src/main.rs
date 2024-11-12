use std::{result, str::FromStr, sync::Arc};

mod handlers;
use axum::{extract::{rejection::{JsonRejection, QueryRejection}, Query, State}, http::StatusCode, routing::{get, post}, Json, Router};
use handlers::database::{self, Db};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum TaskStatus {
    Completed,
    NotCompleted,
    InProgress
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Completed => "Completed".to_string(),
            Self::NotCompleted => "Not Completed".to_string(),
            Self::InProgress => "In Progress".to_string()
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
            _ => Err("Not a valid enum".into())
        }
    }
}


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


#[derive(Serialize, Deserialize)]
struct UserQuery {
    username : String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Comment {
    text: String,
    created_at: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    name: String,
    status:  TaskStatus,
    comments : Vec<Comment>
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Category {
    name: String,
    tasks: Vec<Task>
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
    .route("/tasksget", get(get_tasks))
    .route("/taskspost", post(post_tasks))
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
async fn register(State(appstate) : State<Arc<AppState>>, result : Result<Query<RegData>, QueryRejection>) -> Result<(), (StatusCode, String)> {
    match result {
        Ok(Query(result)) => {
            if !result.name.is_empty() && !result.password.is_empty() {
                
                match appstate.db.add_user(result.name, result.password, result.email).await {
                    Ok(()) => {
                       
                        Ok(())
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
async fn get_tasks(State(appstate) : State<Arc<AppState>>, result : Result<Query<UserQuery>, QueryRejection>) -> Result<Json<Vec<Category>>, (StatusCode, String)> {
    match result {
        Ok(Query(result)) => {
            if !result.username.is_empty() {
                match appstate.db.fetch_tasks(result.username).await {
                    Ok(vec) => {
                        return Ok(Json(vec))
                    }
                    Err(why) => {
                        println!("TAsk fetch error: {}", why);
                        return Err((StatusCode::BAD_REQUEST, "wrong data".into()))
                    }
                }
            } else {
                return Err((StatusCode::BAD_REQUEST, "Wrong creds".into()))
            }
        }
        Err(_) => {
            println!("Incorrect data");
            return Err((StatusCode::NO_CONTENT, "".to_string()))
        }
    }
}
async fn post_tasks(State(appstate) : State<Arc<AppState>>, query : Result<Query<UserQuery>, QueryRejection>, result : Result<Json<Vec<Category>>, JsonRejection>) -> Result<(), (StatusCode, String)> {
    match result {
        Ok(Json(result)) => {
            
             for element in result {
                match appstate.db.add_category(Query(query.as_ref().unwrap()).username.clone(), element).await {
                    Ok(()) => {
                        
                    }
                    Err(why) => {
                        println!("Saving task error {}", why);
                    }
                }
             }
             return Ok(())
        }
        Err(_) => {
            println!("Incorrect data");
            return Err((StatusCode::NO_CONTENT, "".to_string()))
        }
    }
}

