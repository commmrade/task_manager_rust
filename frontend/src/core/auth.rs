use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use super::{app::MyApp, network::{post_request_json}};


#[derive(Deserialize, Serialize)]
pub struct LoginData {
    pub name: String,
    pub password: String
}
#[derive(Deserialize, Serialize)]
pub struct RegData {
    pub name: String,
    pub password: String,
    pub email : String
}



impl MyApp {
    pub fn login(&mut self) {

        let rt = tokio::runtime::Runtime::new().unwrap();
    
        let url = "http://localhost:3000/login";
        
        let login_data = LoginData{name: self.login.clone(), password: self.password.clone()};
        
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    
        match rt.block_on(post_request_json(url, HashMap::new(), headers, serde_json::to_string(&login_data).unwrap())) {
            Ok(txt) => {
                self.token = txt;
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

    pub fn register(&mut self) {
        let url = "http://localhost:3000/register";
                
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        

        let reg_d = RegData{email: self.email.clone(), name: self.login.clone(), password: self.password.clone()};

        match self.rt.block_on(post_request_json(url, HashMap::new(), headers, serde_json::to_string(&reg_d).unwrap())) {
            Ok(txt) => {
                self.token = txt;
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