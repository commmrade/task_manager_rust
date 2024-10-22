
use sqlx::{mysql::MySqlRow, Error, MySql};
use tokio::runtime::Runtime;
use sqlx::Row;

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
    pub async fn user_exists(&self, username : String) -> Result<bool, sqlx::Error> {
        println!("user eists {}", username.len());
        let row = match sqlx::query("SELECT * FROM users WHERE username = ?").bind(username)
        .fetch_one(&self.pool).await {
            Ok(row) => row,
            Err(why) => {
                return Ok(false);
            }
        };

        Ok(!row.is_empty())
    }
    pub async fn add_user(&self, username : String, password : String) -> Result<(), sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok(b_exists) => {
                if !b_exists {
                    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)").bind(username).bind(password)
                    .execute(&self.pool).await.unwrap();
                } else {
                    return Err(sqlx::Error::AnyDriverError("User already exists".into()))
                }       
            }
            Err(why) => {
                return Err(why)
            }
        }
        Ok(())
    }
    pub async fn login_user(&self, username : String, password : String) -> Result<(), sqlx::Error> {

        match self.user_exists(username.clone()).await {
            Ok(b_exists) => {
                println!("jDFJDFJDFJKDFSLJKDSFJKLDFDFSJKLDFLKDFSLK {}", b_exists);
                if b_exists {
                    let row = sqlx::query("SELECT username, password FROM users WHERE username = ?").bind(username.clone())
                    .fetch_one(&self.pool).await?;
                    let name_db : String = row.get(0);
                    let pswd_db : String = row.get(1);
                    println!("{} = {} gkfgflfdgkl", password, pswd_db);
                    if username == name_db && pswd_db == password {
                        return Ok(());
                    } else {
                        return Err(sqlx::Error::AnyDriverError("Data is incorrect".into()));
                    }
                }
            }
            Err(why) => {
                println!("jfdjfjfdfjf");
                return Err(why);
            }
        }
        Err(sqlx::Error::AnyDriverError("user doesnt exist".into()))
    }
}