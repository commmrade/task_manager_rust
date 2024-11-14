
use std::{collections::HashMap, process::id, str::FromStr};

use chrono::{DateTime, Local, Utc};
use sqlx::{mysql::MySqlRow, Error, MySql};
use tokio::runtime::Runtime;
use sqlx::Row;

use crate::{Category, Comment, Task, TaskStatus};

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
    pub async fn category_exists(&self, username : String, title : String) -> Result<(bool, i32), sqlx::Error> {
        let row = match sqlx::query("SELECT * FROM categories WHERE (title = ? AND user_id = ?)")
        .bind(title)
        .bind(self.user_exists(username).await?.1)
        .fetch_one(&self.pool).await {
            Ok(row) => row,
            Err(why) => {
                println!("Error fetching {}", why);
                return Ok((false, 0))
            }
        };
        
        Ok((!row.is_empty(), row.get(0)))
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
    
    pub async fn fetch_comments(&self, task_id : i32) -> Result<Vec<Comment>, sqlx::Error> {
        let mut result : Vec<Comment> = Vec::new();
        let rows = sqlx::query("SELECT text, created_at FROM comments WHERE (task_id = ?)")
        .bind(task_id)
        .fetch_all(&self.pool).await?;
        
        for row in rows {
            let timestamp : sqlx::types::chrono::DateTime<Local> = row.get(1);
            result.push(Comment { text: row.get(0), created_at: timestamp.to_string()});
        }

        Ok(result)

    }
    pub async fn fetch_tasks(&self, username : String) -> Result<Vec<Category>, sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, id)) => {
                if b_exists {
                    let mut cats : HashMap<String, Vec<Task>> = HashMap::new();
                    let mut categories : Vec<Category> = Vec::new();
                    println!("here");
                   

                    let task_rows = sqlx::query("SELECT tasks.title, tasks.status, tasks.id, categories.title FROM tasks LEFT JOIN categories on tasks.category_id = categories.id WHERE tasks.user_id = ?").bind(id)
                    .fetch_all(&self.pool).await?;
                    
                    for task_row in task_rows {
                        cats.entry(task_row.get::<String, _>(3)).or_insert_with(Vec::new).push(Task { name: task_row.get(0), status: TaskStatus::from_str(task_row.get(1)).unwrap(), comments: vec![] });


                        let comms = match self.fetch_comments(task_row.get(2)).await {
                            Ok(vec) => vec, 
                            Err(why) => {
                                println!("Fetching comments error");
                                return Err(why);
                            }
                        };
                        cats.entry(task_row.get::<String, _>(3)).and_modify(|v| v.last_mut().unwrap().comments.extend(comms));
                    }

                    for (category_name, tasks) in cats {
                       
                        categories.push(Category { name: category_name, tasks: tasks });
                    }

                    Ok(categories)
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
    pub async fn add_category(&self, username : String, category : Category) -> Result<(), sqlx::Error> {
        match self.user_exists(username.clone()).await {
            Ok((b_exists, id_u)) => {
                match self.category_exists(username.clone(), category.name.clone()).await {
                    Ok((exists, id_c)) => {
                        
                        if exists {
                            
                            sqlx::query("DELETE FROM tasks WHERE user_id = ?")
                            .bind(id_u)
                            .execute(&self.pool).await?;
                            sqlx::query("DELETE FROM categories WHERE user_id = ?")
                            .bind(id_u)
                            .execute(&self.pool).await?;

                            
                            sqlx::query("INSERT INTO categories (user_id, title) VALUES (?, ?)")
                            .bind(id_u)
                            .bind(category.name.clone()).execute(&self.pool).await?;

                            
                            let row = sqlx::query("SELECT id FROM categories WHERE (user_id = ? AND title = ?)")
                            .bind(id_u)
                            .bind(category.name.clone()).fetch_one(&self.pool).await?;

                            
                            for task in category.tasks {
                                sqlx::query("INSERT INTO tasks (user_id, title, status, category_id) VALUES (?, ?, ?, ?)")
                                .bind(id_u)
                                .bind(task.name.clone())
                                .bind(task.status.to_string())
                                .bind(row.get::<i32, _>(0))
                                .execute(&self.pool).await?;
                               

                                for comm in task.comments {
                                    self.add_comment(username.clone(), task.name.clone(), comm.text).await.unwrap();
                                }   
                            }

                            
                        } else {
                            sqlx::query("INSERT INTO categories (user_id, title) VALUES (?, ?)")
                            .bind(id_u)
                            .bind(category.name.clone()).execute(&self.pool).await?;
                            


                            let row = sqlx::query("SELECT id FROM categories WHERE (user_id = ? AND title = ?)")
                            .bind(id_u)
                            .bind(category.name.clone()).fetch_one(&self.pool).await?;

                            
                            for task in category.tasks {
                                sqlx::query("INSERT INTO tasks (user_id, title, status, category_id) VALUES (?, ?, ?, ?)")
                                .bind(id_u)
                                .bind(task.name.clone())
                                .bind(task.status.to_string())
                                .bind(row.get::<i32, _>(0))
                                .execute(&self.pool).await?;
                               

                                for comm in task.comments {
                                    self.add_comment(username.clone(), task.name.clone(), comm.text).await.unwrap();
                                }   
                            }
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
        match self.task_exists(username, title).await {
            Ok((b_exists, task_id)) => {
                sqlx::query("INSERT INTO comments (task_id, text) VALUES (?, ?)").bind(task_id)
                .bind(comment)
                .execute(&self.pool).await?;

                Ok(())
            }
            Err(why) => {
                return Err(sqlx::Error::AnyDriverError("Task does not exist".into()))
            }
        }
    }
    
}