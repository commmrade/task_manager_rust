use std::{collections::{HashMap, HashSet}, thread, time::Duration};

use egui::{Color32, ComboBox, RichText, Ui};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use super::json_handler;


#[derive(Serialize, Deserialize, PartialEq)]
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


#[derive(Serialize, Deserialize)]
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
    exit_window : bool,
    input_text: String,
    rt : Runtime
}


impl Default for MyApp {
    fn default() -> Self {
        MyApp {tasks: vec![], can_exit: false, exit_window: false, 
        input_text: String::new(), current_user: None,
        token: String::new(), login : String::new(), password: String::new(),
        blogin: true, rt: Runtime::new().unwrap() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_user.is_some() {
                self.draw_tasks_ui(ui, ctx);
            } else {
                self.draw_auth_ui(ui, ctx);
                
            }
            
            //let rt = Runtime::new().unwrap();
            // let lgn = self.login.clone();
            // self.rt.spawn(async move {
            //     tokio::time::sleep(Duration::new(2, 0)).await;
            //     println!("{}", lgn);
            // }); 
            
        });
    }
}

async fn login_user(login : String, password : String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = "http://localhost:3000/login";
    let mut query_params = HashMap::new();
    query_params.insert("name", login);
    query_params.insert("password", password);

    let response = client.get(url).query(&query_params).send().await?;


    if response.status().is_success() {
        return Ok(())
    } else {
        return Err("Err".into());
    }

}


impl MyApp {
    fn remove_task(&mut self, idx: usize) {
        self.tasks.remove(idx);
    }
    fn add_task(&mut self, name: String) {
        self.tasks.push(Task { name: name, status: TaskStatus::NotCompleted });
        
    }
    
    fn handle_input(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        if ui.input(|i| i.viewport().close_requested()) {
            if self.can_exit {

            } else {
                self.exit_window = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            }
        }
    }
    fn confirm_exit_win(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        if self.exit_window {
            egui::Window::new("Confirm exit").collapsible(false).resizable(false).current_pos([ui.available_width() / 2.0, ui.available_height() / 2.0]).show(ctx, |ui| {
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
    fn display_tasks(&mut self, ui : &mut Ui) {
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            for idx in (0..self.tasks.len()).rev() {
           
                ui.horizontal(|ui| {
                    let str = format!("{}.", self.tasks.len() - idx);
                    let text = RichText::new(str).size(16.0).color(Color32::from_rgb(200, 200, 200));
                    ui.label(text);

                    ui.add_space(30.0);

                    let str = format!("Name: {}", self.tasks[idx].name);
                    let text = RichText::new(str).size(16.0).color(Color32::from_rgb(200, 200, 200));
                    ui.label(text);


                    
                    ui.add_space(40.0);

                    egui::ComboBox::from_id_salt(idx.to_string()).selected_text(self.tasks[idx].status.to_string()).show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.tasks[idx].status, TaskStatus::Completed, "Completed");
                        ui.selectable_value(&mut self.tasks[idx].status, TaskStatus::InProgress, "In progress");
                        ui.selectable_value(&mut self.tasks[idx].status, TaskStatus::NotCompleted, "Not completed");
                        
                    });
                    //println!("{}", self.tasks[idx].status.to_string());

                   
                    ui.add_space(30.0);

                    if ui.button("X").clicked() {
                        self.remove_task(idx);   
                    }
                });
                ui.separator();
                

            }
        });
    }
    fn draw_tasks_ui(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        //Heading
        let str = "TODOLIST".to_string();
        let text = RichText::new(str).size(35.0);
        ui.heading(text);
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
        let str = format!("Please authorize");
        let text = RichText::new(str).size(16.0);
        ui.label(text);


        if ui.button("Change").clicked() {
            self.blogin = !self.blogin;
            self.login.clear();
            self.password.clear();
        }

        if self.blogin {
            let str = format!("Login");
            let text = RichText::new(str).size(16.0);
            let labl = ui.label(text);
            ui.text_edit_singleline(&mut self.login).labelled_by(labl.id);

            let str = format!("Password");
            let text = RichText::new(str).size(16.0);
            let labl = ui.label(text);
            ui.text_edit_singleline(&mut self.password).labelled_by(labl.id);

            let log_btn = ui.button("Login");
            if log_btn.clicked() {
                //LOGIN LOGC
                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(login_user(self.login.clone(), self.password.clone())) {
                    Ok(()) => {
                        self.current_user = Some(self.login.clone());
                    }
                    Err(_) => {
                        println!("Try again");
                    }                    
                }


            } else if log_btn.clicked() {
                println!("Incorrect data");
            }
        } else {
            let str = format!("Login");
            let text = RichText::new(str).size(16.0);
            let labl = ui.label(text);
            ui.text_edit_singleline(&mut self.login).labelled_by(labl.id);

            let str = format!("Password");
            let text = RichText::new(str).size(16.0);
            let labl = ui.label(text);
            ui.text_edit_singleline(&mut self.password).labelled_by(labl.id);
            
            let reg_btn = ui.button("Register");
            if reg_btn.clicked() {
                //REG LOGIC

                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(login_user(self.login.clone(), self.password.clone())) {
                    Ok(()) => {
                        self.current_user = Some(self.login.clone());
                    }
                    Err(_) => {
                        println!("Try again");
                    }                    
                }
            }
        }


    }
}