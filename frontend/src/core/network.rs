use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};



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



pub async fn get_request(url : &str, query : &HashMap<String, String>, headers : HeaderMap) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get(url).headers(headers).query(query).send().await?;

    if response.status().is_success() {
        return Ok(response.text().await?)
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
pub async fn post_request(url : &str, query : HashMap<String, String>, headers : HashMap<String, String>, body : HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.post(url).query(&query).send().await?; 

    if response.status().is_success() {
        return Ok(response.text().await?)
    } else if response.status().as_u16() == 204 {
        //To handle incorrect data sent
        return Err("204".into());
    } else if response.status().as_u16() == 400 {
        //Handle wrong creds sent
        return Err("400".into());
    } else if response.status().as_u16() == 401 {
        return Err("401".into())
    } else {
        //Other kinda errors if they magically appear
        return Err("0".into())
    }
}
pub async fn post_request_json(url : &str, query : HashMap<String, String>, headers : HeaderMap, js : String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.post(url).headers(headers).query(&query).body(js.clone()).send().await?;
    
    if response.status().is_success() {
        return Ok(response.text().await?)
    } else if response.status().as_u16() == 204 {
        //To handle incorrect data sent
        return Err("204".into());
    } else if response.status().as_u16() == 400 {
        //Handle wrong creds sent
        return Err("400".into());
    } else if response.status().as_u16() == 401 {
        return Err("401".into())
    }  else {
        //Other kinda errors if they magically appear
        return Err("0".into())
    }
}