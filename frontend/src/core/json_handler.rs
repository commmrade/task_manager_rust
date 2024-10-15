use std::{fs::File, io::{Read, Write}};



pub fn save_json<T>(vec: &Vec<T>, path: &str)
    where T : serde::Serialize
{
    let mut file = File::create(path).unwrap();
    let js = serde_json::to_string_pretty(&vec);
    file.write_all(js.unwrap().as_bytes()).unwrap();

    file.flush().unwrap();
}



pub fn load_json<T>(vec: &mut Vec<T>, path: &str) -> Result<(), String> {
    let mut file = match File::open(path) {
        Ok(file) => file, 
        Err(why) => return Err("File doesn't exist".to_string())
    };
    let mut file_contents = String::new(); 

    file.read_to_string(&mut file_contents).unwrap();

    println!("{}", file_contents);

    Ok(())
}