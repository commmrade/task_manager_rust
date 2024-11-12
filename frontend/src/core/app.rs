use std::{borrow::{Borrow, BorrowMut}, clone, collections::{HashMap, HashSet}, hash::Hash, process::exit, str::FromStr, thread, time::{Duration, SystemTime}};

use chrono::{DateTime, Local, Utc};
use egui::{Color32, ComboBox, RichText, Ui};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;

use super::json_handler;


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Comment {
    text: String,
    created_at: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    name: String,
    status: TaskStatus,
    comments : Vec<Comment>
    
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Category {
    name: String,
    tasks: Vec<Task>
}


pub struct MyApp {
    categories : Vec<Category>,
    can_exit : bool,
    current_user : Option<String>,
    comment_input : String,
    token : String,
    login : String,
    blogin : bool,
    password : String,
    email : String,
    exit_window : bool,
    input_text: String,
    category_input : String,
    rt : Runtime,
    prev_check : DateTime<Local>
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

impl eframe::App for MyApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting app... {}", self.login);
        let url = "http://localhost:3000/tasks";
        let mut query_params: HashMap<String, String> = HashMap::new();
        query_params.insert("username".to_string(), self.current_user.clone().unwrap());
        
        match self.rt.block_on(post_request_json(url, query_params, serde_json::to_string(&self.categories).unwrap())) {
            Ok(()) => {
                
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
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = Color32::from_gray(30);  // Button background
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 150);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_user.is_some() {
                self.draw_tasks_ui(ui, ctx);

                
                if (Local::now() - self.prev_check).num_seconds() >= 30 {
                    self.prev_check = Local::now();
                    

                    let url = "http://localhost:3000/tasks";
                    let mut query_params: HashMap<String, String> = HashMap::new();
                    query_params.insert("username".to_string(), self.current_user.clone().unwrap());
                    let s = serde_json::to_string(&self.categories).unwrap();
                    self.rt.spawn(async move {
                        post_request_json(url, query_params, s).await.unwrap();
                    });
                }

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
async fn post_request(url : &str, query : HashMap<String, String>, headers : HashMap<String, String>, body : HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
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
async fn post_request_json(url : &str, query : HashMap<String, String>, js : String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.post(url).header("Content-Type", "application/json").query(&query).body(js).send().await?;

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
    
    fn remove_task(&mut self, idx_task: usize, idx_category : usize) {
    
        self.categories[idx_category].tasks.remove(idx_task);
        self.categories[idx_category].tasks.shrink_to_fit();
        
    }
    fn add_comment(&mut self, idx_task : usize, idx_category : usize, name : String,) {
        let now = Utc::now();
        let localized_time : DateTime<Local> = DateTime::from(now);
        self.categories[idx_category].tasks[idx_task].comments.push(Comment { text: name, created_at: localized_time.to_string()});
        

    }
    fn update_task(&mut self, idx_task : usize, idx_category : usize, status : TaskStatus) {

        self.categories[idx_category].tasks[idx_task].status = status.clone();

    }
    fn add_task(&mut self, name : String, idx_category : usize) {
        if self.categories[idx_category].tasks.iter().any(|el| el.name == name) {
            println!("Error adding task");
            return;
        }

        self.categories[idx_category].tasks.push(Task { name: name.clone(), status: TaskStatus::NotCompleted, comments : vec![] });
    }
    fn add_category(&mut self, name : String) {
        if self.categories.iter().any(|el| el.name == name) {
            println!("Error adding category");
            return;
        }
        self.categories.push(Category { name: name, tasks: vec![] });
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
            let url = "http://localhost:3000/tasks";
            let mut query_maps = HashMap::new();

            query_maps.insert("username", self.current_user.clone().unwrap());

            let response = client.get(url).query(&query_maps).body(serde_json::to_string_pretty(&self.categories).unwrap()).send().await.unwrap();
            println!("loading...");
            if response.status().is_success() {
                let txt = response.text().await.unwrap();
                let tsks : Vec<Category> = serde_json::from_str(&txt).unwrap();
                // self.tasks.extend(tsks);
                self.categories = tsks;
                //todo!()

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
                
               // let idx_c = idx_category.clone();

                
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
                            ComboBox::from_id_salt(10 * idx_category + idx_task + 100)
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
            self.categories.remove(idx);
            self.categories.shrink_to_fit();
        } 
    }
    
    
    fn draw_tasks_ui(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        


        //Heading
        ui.heading(make_rich_text("TODO List", None));
        ui.add_space(30.0);


        let add_new_text = RichText::new("Add new task:").size(16.0);
        ui.label(add_new_text);
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.category_input);
            if ui.button("Add new category").clicked() {
                self.add_category(self.category_input.clone());

                self.category_input.clear();
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
            egui::TextEdit::singleline(&mut self.login).hint_text("Your username").show(ui);

            
            let labl = ui.label(make_rich_text("Password", None));
            egui::TextEdit::singleline(&mut self.password).password(true).hint_text("Password").show(ui);
            //ui.text_edit_singleline(&mut self.password).labelled_by(labl.id);

            let log_btn = ui.button("Login");
            if log_btn.clicked() {
                //LOGIN LOGC
                let rt = tokio::runtime::Runtime::new().unwrap();

                let url = "http://localhost:3000/login";
                let mut json_body: HashMap<String, String> = HashMap::new();
                json_body.insert("name".to_string(), self.login.clone());
                json_body.insert("password".to_string(), self.password.clone());
                match rt.block_on(post_request(url, HashMap::new(), HashMap::new(), json_body)) {
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
                
                let mut json_body: HashMap<String, String> = HashMap::new();
                json_body.insert("name".to_string(), self.login.clone());
                json_body.insert("password".to_string(), self.password.clone());
                json_body.insert("email".to_string(), self.email.clone());
                match rt.block_on(post_request(url, HashMap::new(), HashMap::new(), json_body)) {
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