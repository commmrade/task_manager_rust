use std::{borrow::{Borrow, BorrowMut}, clone, collections::{HashMap, HashSet}, hash::Hash, process::exit, str::FromStr, sync::{Arc, Mutex}, thread, time::{Duration, SystemTime}};

use chrono::{DateTime, Local, Utc};
use egui::{Color32, ComboBox, RichText, Ui};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;

use crate::core::network::{get_request, post_request_json};

use super::{network::{LoginData, RegData}, task_handler::{Category, TaskStatus}};



pub struct MyApp {
    pub categories : Vec<Category>,
    pub can_exit : bool,
    pub current_user : Option<String>,
    pub comment_input : String,
    pub token : String,
    pub login : String,
    pub blogin : bool,
    pub password : String,
    pub email : String,
    pub exit_window : bool,
    pub input_text: String,
    pub category_input : String,
    pub rt : Runtime,
    pub prev_check : DateTime<Local>
}


impl Default for MyApp {
    fn default() -> Self {
        let app = MyApp {categories: vec![], can_exit: false, exit_window: false, 
            input_text: String::new(), category_input: String::new(), current_user: None,
            token: String::new(), login : String::new(), password: String::new(), 
            blogin: true, rt: Runtime::new().unwrap(), email: String::new(), comment_input: String::new(), prev_check: Local::now() };
        app
    }
}

pub fn make_rich_text(str : &str, font_size: Option<f32>) -> RichText {
    RichText::new(str).size(font_size.unwrap_or(16.0))
}



impl eframe::App for MyApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting app... {}", self.login);
        let url = "http://localhost:3000/tasks";
        let mut query_params: HashMap<String, String> = HashMap::new();
        query_params.insert("username".to_string(), self.current_user.clone().unwrap());
        
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("Authentication", HeaderValue::from_str(&self.token).unwrap());
        
        println!("Categories left {}", self.categories.len());
        match self.rt.block_on(post_request_json(url, query_params, headers, serde_json::to_string_pretty(&self.categories).unwrap())) {
            Ok(_txt) => {
                
            }
            Err(err) => {
                match err.to_string().as_str() {
                    "204" => {
                        println!("Incorrect data for request");
                    }
                    "400" => {
                        println!("Wrong credentials")
                    }
                    "401" => {
                        println!("Wrong token")
                    }
                    _ => {
                        println!("Server is dead");
                    }
                }
            }                    
        }
    }
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = Color32::from_gray(30);  // Button background
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 150);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_user.is_some() {
                self.draw_tasks_ui(ui, ctx);

                
                if (Local::now() - self.prev_check).num_seconds() >= 30 { // If 30 seconds have passed since the last auto-update
                    self.prev_check = Local::now();
                    

                    let url = "http://localhost:3000/tasks";
                    let mut query_params: HashMap<String, String> = HashMap::new();
                    query_params.insert("username".to_string(), self.current_user.clone().unwrap());
                    let s = serde_json::to_string(&self.categories).unwrap();

                    let mut headers : HeaderMap = HeaderMap::new();
                    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
                    headers.insert("Authentication", HeaderValue::from_str(&self.token).unwrap());
                   
                    
                    self.rt.spawn(async move {
                        match post_request_json(url, query_params, headers, s).await {
                            Ok(err) => {
                                if err == "401" {
                                    eprintln!("Token unauthorized");
                                    exit(-1);
                                }
                            }
                            Err(why) => {
                                println!("Saving tasks error: {}", why);
                            }

                        }
                    });
                }

            } else {
                self.draw_auth_ui(ui, ctx);
                
            }
           
            
        });
    }
}

impl MyApp {
    
    pub fn display_tasks(&mut self, ui: &mut Ui) {
       
        let mut tasks_to_remove = Vec::new();
        let mut comments_to_add = Vec::new();
        let mut task_new_name : (String, usize) = (String::new(), 0);
        let mut category_to_remove : Option<usize> = None;

        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            for (idx_category, category) in self.categories.iter_mut().enumerate() {

                ui.horizontal(|ui| {
                    ui.label(make_rich_text(&category.name, 20.0.into()));
                    ui.add_space(15.0);
                    let rem = ui.button("Remove category");
                    if rem.clicked() {
                        category_to_remove = idx_category.into();
                    }
                });
                    
                ui.horizontal(|ui: &mut Ui| {
                    ui.text_edit_singleline(&mut self.input_text);
                    if ui.button("➕ Add Task").clicked() && !self.input_text.is_empty() {
    
                        task_new_name.0 = self.input_text.clone();
                        task_new_name.1 = idx_category;

                        self.input_text.clear(); 
                    }
                });

                let len_cat = category.tasks.len();


                for (idx_task, task) in category.tasks.iter_mut().enumerate() {
                    if idx_task < len_cat {
                        ui.horizontal(|ui| {
                            ui.add_space(30.0);
                            let text = RichText::new(&task.name)
                                .size(16.0)
                                .color(Color32::from_rgb(200, 200, 200));
                            ui.label(text);

                            let prev_status = task.status.clone();
                            ComboBox::from_id_salt(10 * idx_category + idx_task + 10)
                                .selected_text(task.status.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut task.status, TaskStatus::Completed, "Completed");
                                    ui.selectable_value(&mut task.status, TaskStatus::InProgress, "In Progress");
                                    ui.selectable_value(&mut task.status, TaskStatus::NotCompleted, "Not Completed");
                                    if prev_status != task.status.clone() {
                                        // Handle status change if needed
                                    }
                                });

                            if ui.button("❌").clicked() {
                                tasks_to_remove.push((idx_category, idx_task));
                            }
                        });

                        egui::CollapsingHeader::new("Comments")
                            .id_salt(100 * idx_category + idx_task + 1000)
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.add_space(10.0);
                                for comment in &task.comments {
                                    ui.horizontal(|ui| {
                                        ui.label(comment.text.clone());
                                        ui.add_space(15.0);
                                        let time_text = if comment.created_at.contains(".") {
                                            &comment.created_at[..comment.created_at.rfind(".").unwrap()]
                                        } else {
                                            &comment.created_at[..comment.created_at.rfind(" ").unwrap()]
                                        };
                                        ui.label(make_rich_text(time_text, 10.0.into()));
                                    });
                                }

                                ui.horizontal(|ui| {
                                    ui.text_edit_singleline(&mut self.comment_input);
                                    if ui.button("Add comment").clicked() {
                                        comments_to_add.push((idx_category, idx_task, self.comment_input.clone()));
                                        self.comment_input.clear();
                                    }
                                });
                            });

                        ui.separator();
                    }
                }
            }
        });

        // Perform the deferred actions
        for (idx_category, idx_task) in tasks_to_remove {
            self.remove_task(idx_task, idx_category);
        }

        for (idx_category, idx_task, cum) in comments_to_add {
            self.add_comment(idx_task, idx_category, cum);
        }
        if !task_new_name.0.is_empty() {
            self.add_task(task_new_name.0, task_new_name.1);
        }
        if let Some(idx) = category_to_remove {
            if self.categories[idx].tasks.is_empty() {
                self.categories.remove(idx);
                self.categories.shrink_to_fit();
            }
            
        } 
    }
    pub fn confirm_exit_win(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        if self.exit_window {
            egui::Window::new("Confirm exit")
            .collapsible(false)
            .resizable(false)
            .current_pos([ui.available_width() / 2.0, ui.available_height() / 2.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {

                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.exit_window = false;
                        self.can_exit = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Cancel").clicked() {
                        self.exit_window = false;
                    }
                    ui.separator();

                });
            });
        }
    }
    
}

