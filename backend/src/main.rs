use axum::{extract::{rejection::QueryRejection, Query}, http::{Result, StatusCode}, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct QUser {
    name: String,
    age: u32
}


#[tokio::main]
async fn main() {
    println!("hello world");
    let app = 

}

async fn welcome(result : Result<Query<QUser>, QueryRejection>) -> Result<Json<QUser>, (StatusCode, String)> {

    

    Ok(Json(QUser { name: "fuck".to_string(), age: 18 }))
}