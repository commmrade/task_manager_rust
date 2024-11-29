use std::{collections::HashMap, process::exit, str::FromStr};

use chrono::{DateTime, Local, Utc};
use egui::Ui;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};


use crate::core::network::get_request;

use super::app::MyApp;


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum TaskStatus {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub text: String,
    pub created_at: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub name: String,
    pub status: TaskStatus,
    pub comments : Vec<Comment>
    
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub name: String,
    pub tasks: Vec<Task>
}



impl MyApp {
    pub fn remove_task(&mut self, idx_task: usize, idx_category : usize) {
    
        self.categories[idx_category].tasks.remove(idx_task);
        self.categories[idx_category].tasks.shrink_to_fit(); // Deallocating not needed memory
        
    }
    pub fn add_comment(&mut self, idx_task : usize, idx_category : usize, name : String,) {
        let now = Utc::now();
        let localized_time : DateTime<Local> = DateTime::from(now);
        self.categories[idx_category].tasks[idx_task].comments.push(Comment { text: name, created_at: localized_time.to_string()});
        

    }
    pub fn update_task(&mut self, idx_task : usize, idx_category : usize, status : TaskStatus) {

        self.categories[idx_category].tasks[idx_task].status = status.clone();

    }
    pub fn add_task(&mut self, name : String, idx_category : usize) {
        if self.categories[idx_category].tasks.iter().any(|el| el.name == name) {
            println!("Error adding task");
            return;
        }

        self.categories[idx_category].tasks.push(Task { name: name.clone(), status: TaskStatus::NotCompleted, comments : vec![] });
    }
    pub fn add_category(&mut self, name : String) {
        if self.categories.iter().any(|el| el.name == name) {
            println!("Error adding category");
            return;
        }
        self.categories.push(Category { name: name, tasks: vec![] });
    }
    
    pub fn handle_input(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {

        //Confirm window exit
        if ui.input(|i| i.viewport().close_requested()) {
            if self.can_exit {

            } else {
                self.exit_window = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            }
        }
    }

    pub fn load_tasks(&mut self) {
        let usrname = self.login.clone();
        println!("Username to load tasks: {}", &usrname);
        
        let client = reqwest::Client::new();
        let url = "http://localhost:3000/tasks";
        let mut query_maps: HashMap<String, String> = HashMap::new();

        query_maps.insert("username".to_string(), self.current_user.clone().unwrap());

        let mut headers = HeaderMap::new();
        headers.insert("Authentication", HeaderValue::from_str(&self.token).unwrap());

        match self.rt.block_on(get_request(url, &query_maps, headers)) {
            Ok(txt) => {
                let cats : Vec<Category> = serde_json::from_str(&txt).unwrap();
                self.categories = cats;
            }
            Err(why) => {
                if why.to_string().as_str() == "401" {
                    println!("Invalid token");
                    exit(-1);
                }
                println!("Loading task error: {}", why);
            }
        }

    }
}