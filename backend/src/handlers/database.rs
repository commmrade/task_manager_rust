
use std::str::FromStr;

use chrono::{DateTime, Local, Utc};
use sqlx::{mysql::MySqlRow, Error, MySql};
use tokio::runtime::Runtime;
use sqlx::Row;

use crate::{Comment, Task, TaskStatus};

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
    pub async fn task_exists(&self, username : String, title : String) -> Result<(bool, i32), sqlx::Error> {
        let row = match sqlx::query("SELECT * FROM tasks WHERE (title = ? AND user_id = ?)")
        .bind(title)
        .bind(self.user_exists(username).await?.1)
        .fetch_one(&self.pool).await {
            Ok(row) => row,
            Err(why) => {
                println!("Error fetching {}", why);
                return Ok((false, 0))
            }
        };
        println!("task exists {}", row.is_empty());
        Ok((!row.is_empty(), row.get(0)))
    }
    pub async fn add_user(&self, username : String, password : String, email : String) -> Result<(), sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, _)) => {
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
            Ok((b_exists, _)) => {
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
    
    pub async fn fetch_comments(&self, username : String, title : String, username_id : i32, task_id : i32) -> Result<Vec<Comment>, sqlx::Error> {
        let mut result : Vec<Comment> = Vec::new();
        let rows = sqlx::query("SELECT text, created_at FROM comments WHERE (user_id = ? AND task_id = ?)")
        .bind(username_id)
        .bind(task_id)
        .fetch_all(&self.pool).await?;
        
        for row in rows {
            let timestamp : sqlx::types::chrono::DateTime<Local> = row.get(1);
            result.push(Comment { text: row.get(0), created_at: timestamp.to_string()});
        }

        Ok(result)

    }
    pub async fn fetch_tasks(&self, username : String) -> Result<Vec<Task>, sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, id)) => {
                if b_exists {
                    let mut tasks : Vec<Task> = Vec::new();
                    let rows = sqlx::query("SELECT title, status, id FROM tasks WHERE user_id = ?")
                    .bind(id)
                    .fetch_all(&self.pool).await?;

                    for row in rows {
                        let comms = match self.fetch_comments(username.clone(), row.get(1), id, row.get(2)).await {
                            Ok(vec) => vec, 
                            Err(why) => {
                                println!("Fetching comments error");
                                return Err(why);
                            }
                        };
                        let mut task = Task { name: row.get(0), status: TaskStatus::from_str(row.get(1)).unwrap(),comments: vec![] };
                        task.comments.extend(comms);

                        tasks.push(task);
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
    pub async fn add_task(&self, username : String, task : Task) -> Result<(), sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, id_u)) => {
                match self.task_exists(username.clone(), task.name.clone()).await {
                    Ok((exists, id_t)) => {
                        
                        if exists {
                            sqlx::query("DELETE FROM tasks WHERE (user_id = ?)")
                            .bind(id_u)
                            .execute(&self.pool).await?;

                            sqlx::query("INSERT INTO tasks (user_id, title, status) VALUES (?, ?, ?)")
                            .bind(id_u)
                            .bind(task.name.clone())
                            .bind(task.status.to_string())
                            .execute(&self.pool).await?;
                        } else {
                            sqlx::query("INSERT INTO tasks (user_id, title, status) VALUES (?, ?, ?)")
                            .bind(id_u)
                            .bind(task.name.clone())
                            .bind(task.status.to_string())
                            .execute(&self.pool).await?;
                        }

                        for comm in task.comments {
                            self.add_comment(username.clone(), task.name.clone(), comm.text).await.unwrap();
                        }
                    }
                    Err(why) => {
                        println!("Why {}", why);
                        return Err(why)
                    }
                }


                Ok(())
            }
            Err(why) => {
                println!("Fetching error {}", why);
                return Err(why)
            }
        }
    }
    pub async fn add_comment(&self, username : String, title : String, comment : String) -> Result<(), sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, id)) => {
                if b_exists {
                    match self.task_exists(username, title).await {
                        Ok((b_exists, task_id)) => {
                            sqlx::query("INSERT INTO comments (task_id, user_id, text) VALUES (?, ?, ?)").bind(task_id)
                            .bind(id)
                            .bind(comment)
                            .execute(&self.pool).await?;

                            Ok(())
                        }
                        Err(why) => {
                            return Err(sqlx::Error::AnyDriverError("Task does not exist".into()))
                        }
                    }
                   
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