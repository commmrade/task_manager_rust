
use sqlx::{Error, MySql};
use tokio::runtime::Runtime;


pub struct Db {
    pool : sqlx::Pool<MySql>
}

impl Db
{
    pub async fn new(url : String) -> Result<Self, Error> {
    
       
        let pool = sqlx::MySqlPool::connect(&url).await.unwrap();

        Ok(Self { pool })
    }
    pub fn print(&self) {
        println!("Lox ebaniy");
    }
}