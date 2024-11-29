use std::{collections::HashMap, process::id, str::FromStr};

use chrono::{DateTime, Local, Utc};
use sqlx::Row;
use sqlx::{mysql::MySqlRow, Error, MySql};
use tokio::runtime::Runtime;

use crate::handlers::tasks::{Category, Comment, Task, TaskStatus};





pub struct Db {
    pool: sqlx::Pool<MySql>,
}

impl Db {
    pub async fn new(url: String) -> Result<Self, Error> {
        let pool = sqlx::MySqlPool::connect(&url).await.map_err(|e| e.to_string());

        Ok(Self { pool: pool.unwrap() })
    }

    pub async fn user_exists(&self, username: String) -> Result<Option<i32>, sqlx::Error> {
        let row = sqlx::query("SELECT id FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.get(0)))
    }

    pub async fn category_exists(
        &self,
        username: String,
        title: String,
    ) -> Result<Option<i32>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM categories WHERE (title = ? AND user_id = ?)")
            .bind(title)
            .bind(self.user_exists(username).await?.unwrap())
            .fetch_optional(&self.pool)
            .await?;

        let id: Option<i32> = row.map_or(None, |r| r.get(0));

        Ok(id)
    }

    pub async fn task_exists(
        &self,
        username: String,
        title: String,
    ) -> Result<Option<i32>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM tasks WHERE (title = ? AND user_id = ?)")
            .bind(title)
            .bind(self.user_exists(username).await?.unwrap())
            .fetch_optional(&self.pool)
            .await?;

        let task = row.map_or(None, |r| r.get(0));

        Ok(task)
    }


    pub async fn add_user(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> Result<(), sqlx::Error> {
        let user = self.user_exists(username.clone()).await?;
        if let None = user {
            sqlx::query("INSERT INTO users (username, password, email) VALUES (?, ?, ?)")
                .bind(username)
                .bind(password)
                .bind(email)
                .execute(&self.pool)
                .await
                .unwrap();
            return Ok(());
        }
        return Err(sqlx::Error::AnyDriverError("User already exists".into()));
    }


    pub async fn login_user(&self, username: String, password: String) -> Result<(), sqlx::Error> {
        let user_id = self.user_exists(username.clone()).await?;
        if let Some(_) = user_id {
            let row = sqlx::query("SELECT username, password FROM users WHERE username = ?")
                .bind(username.clone())
                .fetch_one(&self.pool)
                .await?;
            let name_db: String = row.get(0);
            let pswd_db: String = row.get(1);

            if username == name_db && pswd_db == password {
                return Ok(());
            } else {
                return Err(sqlx::Error::AnyDriverError("Data is incorrect".into()));
            }
        }
        return Err(sqlx::Error::AnyDriverError("User does not exist".into()));
    }


    pub async fn fetch_comments(&self, task_id: i32) -> Result<Vec<Comment>, sqlx::Error> {
        let mut result: Vec<Comment> = Vec::new();
        let rows = sqlx::query("SELECT text, created_at FROM comments WHERE (task_id = ?)")
            .bind(task_id)
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            let timestamp: sqlx::types::chrono::DateTime<Local> = row.get(1);
            result.push(Comment {
                text: row.get(0),
                created_at: timestamp.to_string(),
            });
        }

        Ok(result)
    }
    pub async fn fetch_tasks(&self, username: String) -> Result<Vec<Category>, sqlx::Error> {
        let user_id = self.user_exists(username.clone()).await?;

        if let Some(usr_id) = user_id {
            let mut cats: HashMap<String, Vec<Task>> = HashMap::new();
            let mut categories: Vec<Category> = Vec::new();

            let category_rows = sqlx::query("SELECT c.id AS category_id, c.title AS category_title, t.id AS task_id, t.title AS task_title, t.status  FROM categories c LEFT JOIN tasks t ON c.id = t.category_id WHERE c.user_id = ?").bind(usr_id)
            .fetch_all(&self.pool).await?;

            for category_row in category_rows {
                cats.entry(category_row.get::<String, _>(1))
                    .or_insert_with(Vec::new); // Insert empty vec

                if let Ok(name) = category_row.try_get::<String, _>(3) { // If task isn't NULL
                    cats.get_mut(&category_row.get::<String, _>(1)).unwrap().push(Task {
                        name: category_row.get(3),
                        status: TaskStatus::from_str(category_row.get(4)).unwrap(),
                        comments: vec![],
                    });
                    let comms = match self.fetch_comments(category_row.get(2)).await {
                        Ok(vec) => vec,
                        Err(why) => {
                            println!("Fetching comments error");
                            return Err(why);
                        }
                    };
                    // Push comments
                    cats.entry(category_row.get::<String, _>(3))
                        .and_modify(|v| v.last_mut().unwrap().comments.extend(comms));
                }
                
            }

            for (category_name, tasks) in cats { // Convert HashMap to Vec to send as Json then
                categories.push(Category {
                    name: category_name,
                    tasks: tasks,
                });
            } 

            return Ok(categories);
        }
        return Err(sqlx::Error::AnyDriverError("User does not exist".into()));
    }

    pub async fn remove_tasks(&self, username : String) -> Result<(), sqlx::Error> {
        let user = self.user_exists(username).await?;

        if let Some(user_id) = user {
            sqlx::query("DELETE FROM tasks WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
            
            sqlx::query("DELETE FROM categories WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
    pub async fn add_category(
        &self,
        username: String,
        category: Category,
    ) -> Result<(), sqlx::Error> {
        let user = self.user_exists(username.clone()).await?;

        if let Some(user_id) = user {
            let category_obj = self
                .category_exists(username.clone(), category.name.clone())
                .await?;

            if let Some(category_id) = category_obj {
                sqlx::query("INSERT INTO categories (user_id, title) VALUES (?, ?)")
                    .bind(user_id)
                    .bind(category.name.clone())
                    .execute(&self.pool)
                    .await?;
               
                let row =
                    sqlx::query("SELECT id FROM categories WHERE (user_id = ? AND title = ?)")
                        .bind(user_id)
                        .bind(category.name.clone())
                        .fetch_one(&self.pool)
                        .await?;
              
                for task in category.tasks {
                    sqlx::query("INSERT INTO tasks (user_id, title, status, category_id) VALUES (?, ?, ?, ?)")
                    .bind(user_id)
                    .bind(task.name.clone())
                    .bind(task.status.to_string())
                    .bind(row.get::<i32, _>(0))
                    .execute(&self.pool).await?;

                    for comm in task.comments {
                        self.add_comment(username.clone(), task.name.clone(), comm.text)
                            .await
                            .unwrap();
                    }
                }
                
                return Ok(());
            } else {
                sqlx::query("INSERT INTO categories (user_id, title) VALUES (?, ?)")
                .bind(user_id)
                .bind(category.name.clone()).execute(&self.pool).await?;
                

                let row = sqlx::query("SELECT id FROM categories WHERE (user_id = ? AND title = ?)")
                .bind(user_id)
                .bind(category.name.clone()).fetch_one(&self.pool).await?;

                
                for task in category.tasks {
                    sqlx::query("INSERT INTO tasks (user_id, title, status, category_id) VALUES (?, ?, ?, ?)")
                    .bind(user_id)
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
        



        println!("here");
        return Ok(())
    }

    pub async fn add_comment(
        &self,
        username: String,
        title: String,
        comment: String,
    ) -> Result<(), sqlx::Error> {
        let task = self.task_exists(username, title).await?;

        if let Some(task_id) = task {
            sqlx::query("INSERT INTO comments (task_id, text) VALUES (?, ?)")
                .bind(task_id)
                .bind(comment)
                .execute(&self.pool)
                .await?;

            return Ok(());
        }
        Err(sqlx::Error::AnyDriverError("Task does not exist".into()))
    }
}
