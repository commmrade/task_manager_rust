use std::{clone, collections::{HashMap, HashSet}, str::FromStr, thread, time::Duration};

use egui::{Color32, ComboBox, RichText, Ui};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use super::json_handler;


#[derive(Serialize, Deserialize, PartialEq, Clone)]
enum TaskStatus {
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


#[derive(Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    status: TaskStatus
}
pub struct MyApp {
    tasks : Vec<Task>,
    can_exit : bool,
    current_user : Option<String>,
    token : String,
    login : String,
    blogin : bool,
    password : String,
    email : String,
    exit_window : bool,
    input_text: String,
    rt : Runtime
}


impl Default for MyApp {
    fn default() -> Self {
        MyApp {tasks: vec![], can_exit: false, exit_window: false, 
            input_text: String::new(), current_user: None,
            token: String::new(), login : String::new(), password: String::new(),
            blogin: true, rt: Runtime::new().unwrap(), email: String::new() }
    }
}

impl eframe::App for MyApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting app... {}", self.login);
    }
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_user.is_some() {
                self.draw_tasks_ui(ui, ctx);
            } else {
                self.draw_auth_ui(ui, ctx);
                
            }
        });
    }
}
fn make_rich_text(str : &str, font_size: Option<f32>) -> RichText {
    RichText::new(str).size(font_size.unwrap_or(16.0))
}
async fn get_request(url : &str, query : &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get(url).query(query).send().await?;

    if response.status().is_success() {
        return Ok(())
    } else if response.status().as_u16() == 204 {
        //To handle incorrect data sent
        return Err("204".into());
    } else if response.status().as_u16() == 400 {
        //Handle wrong creds sent
        return Err("400".into());
    } else {
        //Other kinda errors if they magically appear
        return Err("0".into())
    }
}
async fn post_request(url : &str, query : HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.post(url).query(&query).send().await?;

    if response.status().is_success() {
        return Ok(())
    } else if response.status().as_u16() == 204 {
        //To handle incorrect data sent
        return Err("204".into());
    } else if response.status().as_u16() == 400 {
        //Handle wrong creds sent
        return Err("400".into());
    } else {
        //Other kinda errors if they magically appear
        return Err("0".into())
    }
}

impl MyApp {
    
    fn remove_task(&mut self, idx: usize) {
        
        
        let username = self.current_user.clone().unwrap();
        let title = self.tasks[idx].name.clone();

        println!("Task removing {}", title);
        
        let url = "http://localhost:3000/taskremove";
        let mut query_params = HashMap::new();
        query_params.insert("username".to_string(), username);
        query_params.insert("title".to_string(), title);
        self.rt.spawn(async move {
            match post_request(url, query_params).await {
                Ok(()) => {
                    println!("Task successfully removed");
                    
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    match err.to_string().as_str() {
                        "204" => {
                            println!("Incorrect data for request");
                        }
                        "400" => {
                            println!("Wrong credentials")
                        }
                        _ => {
                            println!("Server is dead");
                        }
                    }
                    
                }      
            }
        });
        self.tasks.remove(idx);
        
    }
    fn update_task(&mut self, idx: usize, status : TaskStatus) {

        self.tasks[idx].status = status.clone();

        let username = self.current_user.clone().unwrap();
        let title = self.tasks[idx].name.clone();

        let url = "http://localhost:3000/taskupdate";
        let mut query_params = HashMap::new();
        query_params.insert("username".to_string(), username);
        query_params.insert("title".to_string(), title);
        query_params.insert("status".to_string(), status.to_string());
        self.rt.spawn(async move {
            match post_request(url, query_params).await {
                Ok(()) => {
                    println!("Task successfully updated");
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    match err.to_string().as_str() {
                        "204" => {
                            println!("Incorrect data for request");
                        }
                        "400" => {
                            println!("Wrong credentials")
                        }
                        _ => {
                            println!("Server is dead");
                        }
                    }
                    
                }   
            }
        });


    }
    fn add_task(&mut self, name : String) {
        if self.tasks.iter().any(|el| el.name == name) {
            println!("Error adding task");
            return;
        }
        
        self.tasks.insert(0, Task { name: name.clone(), status: TaskStatus::NotCompleted });
        let title = name.clone();
        println!("TASK ADDING {}", title);
        let username = self.current_user.clone().unwrap();

        let url = "http://localhost:3000/taskadd";
        let mut query_params = HashMap::new();
        query_params.insert("username".to_string(), username);
        query_params.insert("title".to_string(), title);
        self.rt.spawn(async move {
            match post_request(url, query_params).await {
                Ok(()) => {
                    println!("Task successfully added");
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    match err.to_string().as_str() {
                        "204" => {
                            println!("Incorrect data for request");
                        }
                        "400" => {
                            println!("Wrong credentials")
                        }
                        _ => {
                            println!("Server is dead");
                        }
                    }
                    
                }      
            }
        });
        
    }
    
    fn handle_input(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {

        //Confirm window exit
        if ui.input(|i| i.viewport().close_requested()) {
            if self.can_exit {

            } else {
                self.exit_window = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            }
        }
    }

    fn load_tasks(&mut self) {
        let usrname = self.login.clone();
        println!("Username to load tasks: {}", &usrname);
        
        self.rt.block_on(async {
            let client = reqwest::Client::new();
            let url = "http://localhost:3000/tasksget";
            let mut query_maps = HashMap::new();

            query_maps.insert("username", self.current_user.clone().unwrap());

            let response = client.get(url).query(&query_maps).send().await.unwrap();
            println!("loading...");
            if response.status().is_success() {
                let txt = response.text().await.unwrap();
                let tsks : Vec<Task> = serde_json::from_str(&txt).unwrap();
                self.tasks.extend(tsks);
                self.tasks.reverse();


            } else if response.status().as_u16() == 204 {
              
            } else if response.status().as_u16() == 400 {
                
            } else {
                 //Other kinda errors if they magically appear
            }

        })
    }


    fn confirm_exit_win(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
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
    fn display_tasks(&mut self, ui: &mut Ui) {
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            for idx in (0..self.tasks.len()).rev() {
                ui.horizontal(|ui| {
                    let str = format!("{}.", self.tasks.len() - idx); 
                    let text = RichText::new(str).size(16.0).color(Color32::from_rgb(200, 200, 200));
                    ui.label(text);
    
                    ui.add_space(30.0);
    
                    let str = format!("Name: {}", self.tasks[idx].name);
                    let text = RichText::new(str).size(16.0).color(Color32::from_rgb(200, 200, 200));
                    
    
                    // ui.add_space(40.0);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max).with_main_justify(true), |ui| {
                        ui.add_space(ui.available_width() / 2.0);
                        let status_before = self.tasks[idx].status.to_string();
                        egui::ComboBox::from_id_salt(idx.to_string())
                            .selected_text(self.tasks[idx].status.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.tasks[idx].status, TaskStatus::Completed, "Completed");
                                ui.selectable_value(&mut self.tasks[idx].status, TaskStatus::InProgress, "In progress");
                                ui.selectable_value(&mut self.tasks[idx].status, TaskStatus::NotCompleted, "Not completed");
                
                                if status_before != self.tasks[idx].status.to_string() {
                                    let new_status = self.tasks[idx].status.clone();
                                    self.update_task(idx, new_status);
                                }
                            });
                    });
                    
    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        ui.add_space(10.0);  // Optional space to separate from other content
                        if ui.button("X").clicked() {
                            self.remove_task(idx);
                        }
                    });
                });
                ui.separator();
            }

           
        });
    }
    fn draw_tasks_ui(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        //Heading
        ui.heading(make_rich_text("TODO List", None));
        ui.add_space(30.0);


        let add_new_text = RichText::new("Add new task:").size(16.0);
        ui.label(add_new_text);
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input_text);
            if ui.button("Add new task").clicked() && !self.input_text.is_empty() {
                self.add_task(self.input_text.clone());

                self.input_text.clear();
            }
        });
        ui.add_space(20.0);


        //Input handling
        self.handle_input(ui, ctx);

        //Confirm exit window
        self.confirm_exit_win(ui, ctx);
       
        //Task shower
        ui.separator();
        ui.vertical(|ui| {
            self.display_tasks(ui);
        });

    }
    fn draw_auth_ui(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        ui.label(make_rich_text("Please authorize", None));

        if ui.button("Change").clicked() {
            self.blogin = !self.blogin;
            self.login.clear();
            self.password.clear();
        }
        if self.blogin {
            let labl = ui.label(make_rich_text("Login", None));
            ui.text_edit_singleline(&mut self.login).labelled_by(labl.id);

            
            let labl = ui.label(make_rich_text("Password", None));
            ui.text_edit_singleline(&mut self.password).labelled_by(labl.id);

            let log_btn = ui.button("Login");
            if log_btn.clicked() {
                //LOGIN LOGC
                let rt = tokio::runtime::Runtime::new().unwrap();

                let url = "http://localhost:3000/login";
                let mut query_params = HashMap::new();
                query_params.insert("name".to_string(), self.login.clone());
                query_params.insert("password".to_string(), self.password.clone());
                match rt.block_on(get_request(url, &query_params)) {
                    Ok(()) => {
                        self.current_user = Some(self.login.clone());
                        println!("load all tasks");
                        self.load_tasks();

                        
                    }
                    Err(err) => {
                        match err.to_string().as_str() {
                            "204" => {
                                println!("Incorrect data for request");
                            }
                            "400" => {
                                println!("Wrong credentials")
                            }
                            _ => {
                                println!("Server is dead");
                            }
                        }
                    }                    
                }


            }
        } else {
            let labl = ui.label(make_rich_text("Email", None));
            ui.text_edit_singleline(&mut self.email).labelled_by(labl.id);

            
            
            let labl = ui.label(make_rich_text("Login", None));
           
            ui.text_edit_singleline(&mut self.login).labelled_by(labl.id);

            // let str = format!("Password");
            // let text = RichText::new(str).size(16.0);
           
            let labl = ui.label(make_rich_text("Password", None));
            
            ui.text_edit_singleline(&mut self.password).labelled_by(labl.id);

            
            
            let reg_btn = ui.button("Register");
            if reg_btn.clicked() {
                //REG LOGIC

                let rt = tokio::runtime::Runtime::new().unwrap();

                let url = "http://localhost:3000/register";
                let mut query_params = HashMap::new();
                query_params.insert("name".to_string(), self.login.clone());
                query_params.insert("password".to_string(), self.password.clone());
                query_params.insert("email".to_string(), self.email.clone());
                match rt.block_on(post_request(url, query_params)) {
                    Ok(()) => {
                        self.current_user = Some(self.login.clone());
                        self.load_tasks();
                 
                    }
                    Err(err) => {
                        println!("{}", err.to_string());
                        match err.to_string().as_str() {
                            "204" => {
                                println!("Incorrect data for request");
                            }
                            "400" => {
                                println!("Wrong credentials")
                            }
                            _ => {
                                println!("Server is dead");
                            }
                        }
                        
                    }                    
                }
            }
        }


    }
}