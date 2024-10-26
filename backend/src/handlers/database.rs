
use std::str::FromStr;

use sqlx::{mysql::MySqlRow, Error, MySql};
use tokio::runtime::Runtime;
use sqlx::Row;

use crate::{Task, TaskStatus};

pub struct Db {
    pool : sqlx::Pool<MySql>
}

impl Db
{
    pub async fn new(url : String) -> Result<Self, Error> {
        let pool = sqlx::MySqlPool::connect(&url).await.unwrap();

        Ok(Self { pool })
    }
    pub async fn user_exists(&self, username : String) -> Result<(bool, i32), sqlx::Error> {
       
        let row = match sqlx::query("SELECT * FROM users WHERE username = ?").bind(username)
        .fetch_one(&self.pool).await {
            Ok(row) => row,
            Err(why) => {
                return Ok((false, 0));
            }
        };

        Ok((!row.is_empty(), row.get(0))) //true if not empty
    }
    pub async fn task_exists(&self, username : String, title : String) -> Result<bool, sqlx::Error> {
        let row = match sqlx::query("SELECT * FROM tasks WHERE (title = ? AND user_id = ?)")
        .bind(title)
        .bind(self.user_exists(username).await?.1)
        .fetch_one(&self.pool).await {
            Ok(row) => row,
            Err(why) => {
                println!("Error fetching {}", why);
                return Ok(false)
            }
        };
        println!("task exists {}", row.is_empty());
        Ok(!row.is_empty())
    }
    pub async fn add_user(&self, username : String, password : String, email : String) -> Result<(), sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, id)) => {
                if !b_exists {
                    sqlx::query("INSERT INTO users (username, password, email) VALUES (?, ?, ?)").bind(username)
                    .bind(password)
                    .bind(email)
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
            Ok((b_exists, id)) => {
                if b_exists {
                    let row = sqlx::query("SELECT username, password FROM users WHERE username = ?").bind(username.clone())
                    .fetch_one(&self.pool).await?;
                    let name_db : String = row.get(0);
                    let pswd_db : String = row.get(1);
                    
                    if username == name_db && pswd_db == password {
                        return Ok(());
                    } else {
                        return Err(sqlx::Error::AnyDriverError("Data is incorrect".into()));
                    }
                } else {
                    return Err(sqlx::Error::AnyDriverError("User does not exist".into()));
                }
            }
            Err(why) => {
                println!("jfdjfjfdfjf");
                return Err(why);
            }
        }
    }
    pub async fn add_task(&self, username : String, title : String) -> Result<(), sqlx::Error> {

        match self.user_exists(username.clone()).await {
            Ok((b_exists, id)) => {
                if b_exists && !self.task_exists(username, title.clone()).await.unwrap() {
                    println!("Adding task");
                    sqlx::query("INSERT INTO tasks (user_id, title, status) VALUES (?, ?, ?)")
                    .bind(id)
                    .bind(title)
                    .bind("Not Completed".to_string())
                    .execute(&self.pool).await?;
                } else {
                    return Err(sqlx::Error::AnyDriverError("User does not exist".into()));
                }
            }
            Err(why) => {
                println!("Adding task error");
                return Err(why);
            }
        }


        Ok(())
    }
    pub async fn remove_task(&self, username : String, title : String) -> Result<(), sqlx::Error> {
        match self.task_exists(username.clone(), title.clone()).await {
            Ok(b_exists) => {
                if b_exists {
                    sqlx::query("DELETE FROM tasks WHERE (title = ? AND user_id = ?)")
                    .bind(title)
                    .bind(self.user_exists(username).await.unwrap().1)
                    .execute(&self.pool).await?;
                } else {
                    return Err(sqlx::Error::AnyDriverError("Task does not exist".into()))
                }
            }
            Err(why) => {
                println!("Removing task error");
                return Err(why);
            }
        }

        Ok(())
    }
    pub async fn update_task(&self, username : String, title : String, status : String) -> Result<(), sqlx::Error> {

        match self.task_exists(username.clone(), title.clone()).await {
            Ok(b_exists) => {
                if b_exists {
                    sqlx::query("UPDATE tasks SET status = ? WHERE (user_id = ? AND title = ?)")
                    .bind(status.to_string())
                    .bind(self.user_exists(username).await.unwrap().1)
                    .bind(title)
                    .execute(&self.pool).await?;
                } else {
                    return Err(sqlx::Error::AnyDriverError("Task does not exist".into()))
                }
            }
            Err(why) => {
                println!("Removing task error");
                return Err(why);
            }
        }

        Ok(())
    }
    pub async fn fetch_tasks(&self, username : String) -> Result<Vec<Task>, sqlx::Error> {
        match self.user_exists(username).await {
            Ok((b_exists, id)) => {
                if b_exists {
                    let mut tasks : Vec<Task> = Vec::new();
                    let rows = sqlx::query("SELECT title, status FROM tasks WHERE user_id = ?")
                    .bind(id)
                    .fetch_all(&self.pool).await?;

                    for row in rows {
                        tasks.push(Task { name: row.get(0), status: TaskStatus::from_str(row.get(1)).unwrap() });
                    }

                    Ok(tasks)
                } else {
                    return Err(sqlx::Error::AnyDriverError("User does not exist".into()))
                }
            }
            Err(why) => {
                println!("Fetching error {}", why);
                return Err(why)
            }
        }
    }
}