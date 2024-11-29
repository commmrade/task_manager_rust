use std::{borrow::{Borrow, BorrowMut}, clone, collections::{HashMap, HashSet}, hash::Hash, process::exit, str::FromStr, sync::{Arc, Mutex}, thread, time::{Duration, SystemTime}};

use chrono::{DateTime, Local, Utc};
use egui::{Color32, ComboBox, RichText, Ui};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;

use crate::core::network::{get_request, post_request_json};

use super::task_handler::{Category, TaskStatus};



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
        MyApp {categories: vec![], can_exit: false, exit_window: false, 
            input_text: String::new(), category_input: String::new(), current_user: None,
            token: String::new(), login : String::new(), password: String::new(), 
            blogin: true, rt: Runtime::new().unwrap(), email: String::new(), comment_input: String::new(), prev_check: Local::now() }
        
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
      
        match self.rt.block_on(post_request_json(url, query_params, headers, serde_json::to_string_pretty(&self.categories).unwrap())) { // Saving everything on exit
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
        let mut task_new_name: (String, usize) = (String::new(), 0);
        let mut category_to_remove: Option<usize> = None; // Defer removals and etc
    
        // Start a scrollable area
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // Iterate over categories
                ui.style_mut().spacing.item_spacing = [10.0, 30.0].into();
                for (idx_category, category) in self.categories.iter_mut().enumerate() {
                    // Category Card
                    ui.group(|ui| {
                        ui.style_mut().spacing.item_spacing = [10.0, 5.0].into(); // Tight spacing
    
                        // Header with category name and remove button
                        ui.horizontal(|ui| {
                            ui.label(make_rich_text(&category.name, 26.0.into()).color(Color32::from_rgb(160, 160, 160)));
                            ui.add_space(10.0);
                            let rem = ui.button("❌ Remove Category");
                            if rem.clicked() {
                                category_to_remove = Some(idx_category);
                            }
                        });
    
                        ui.add_space(15.0); // Space before tasks
    
                        // Task Input Section
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.input_text);
                            if ui.button("➕ Add Task").clicked() && !self.input_text.is_empty() {
                                task_new_name.0 = self.input_text.clone();
                                task_new_name.1 = idx_category;
                                self.input_text.clear();
                            }
                        });
    
                        ui.add_space(15.0); // Space before tasks list
    
                        // Task List (Inside a box for better visual separation)
                        ui.group(|ui| {
                            ui.add_space(10.0); // Spacing for the box
    
                            for (idx_task, task) in category.tasks.iter_mut().enumerate() {
                                ui.group(|ui| {
                                    // Task Card Style
                                    ui.style_mut().spacing.item_spacing = [15.0, 5.0].into(); // Space between elements inside the task
    
                                    // Task Row (Text and Status)
                                    ui.horizontal(|ui| {
                                        ui.add_space(20.0); // Indentation for better visual hierarchy
                                        let text = RichText::new(&task.name)
                                            .size(18.0)
                                            .color(Color32::from_rgb(70, 70, 70)); // Task name color
    
                                        ui.label(text);
    
                                        // ComboBox for task status
                                        let prev_status = task.status.clone();
                                        ComboBox::from_id_salt(format!("category_{}_task_{}_combo", idx_category, idx_task))
                                            .selected_text(task.status.to_string())
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut task.status, TaskStatus::Completed, "Completed");
                                                ui.selectable_value(&mut task.status, TaskStatus::InProgress, "In Progress");
                                                ui.selectable_value(&mut task.status, TaskStatus::NotCompleted, "Not Completed");
                                            });
                                        
                                        // Task Remove Button
                                        if ui.button("❌").clicked() {
                                            tasks_to_remove.push((idx_category, idx_task));
                                        }
                                    });
    
                                    // Comments Section
                                    egui::CollapsingHeader::new("Comments").id_salt(10 * idx_category + idx_task + 100)
                                    .default_open(false) // Set the default open/close state (false means collapsed initially)
                                    .show(ui, |ui| {
                                        // Iterate through the comments and display each one
                                        for comment in &task.comments {
                                            ui.horizontal(|ui| {
                                                ui.label(make_rich_text(&comment.text, 14.0.into()));
                                                ui.add_space(10.0); // Space between text and timestamp

                                                // Extract and format the comment creation time
                                                let time_text = if comment.created_at.contains(".") {
                                                    &comment.created_at[..comment.created_at.rfind(".").unwrap()]
                                                } else {
                                                    &comment.created_at[..comment.created_at.rfind(" ").unwrap()]
                                                };
                                                // Display the timestamp in smaller text
                                                ui.label(make_rich_text(time_text, 12.0.into()).color(Color32::from_gray(150)));
                                            });
                                        }

                                        // Comment Input Field
                                        ui.horizontal(|ui| {
                                            ui.text_edit_singleline(&mut self.comment_input); // Input for new comment

                                            // Button to add the comment
                                            if ui.button("💬 Add Comment").clicked() {
                                                comments_to_add.push((idx_category, idx_task, self.comment_input.clone())); // Add comment
                                                self.comment_input.clear(); // Clear input after adding
                                            }
                                        });
                                    });
    
                                    ui.add_space(10.0); // Space after each task
                                });
                            }
                        });
    
                        ui.add_space(15.0); // Space after tasks box
                    });
                }
            });
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

