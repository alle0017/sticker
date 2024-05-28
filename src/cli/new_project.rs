extern crate colored;
extern crate git2;

use std::{fs::File, io::Write};

use colored::Colorize;

const REPO_URL: &str = "https://github.com/alle0017/spiderweb.js";
const CONFIG_FILE: &str = "/sticker-config.json";
const LIB_DIR: &str = "/spiderweb.js";
const HTML_FILE: &str = "/index.html";
const HTML_BOILERPLATE: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\t\t<meta charset=\"UTF-8\">\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\t\t<title>Document</title>\n\t</head>\n\t<body>\n\t\t<script type=\"module\" src=\"index.js\"></script>\n\t</body>\n</html>";
const JS_FILE: &str = "/index.js";
const JS_CONTENT: &str = "import { define } from './spiderweb.js/api.js';";

fn get_path() -> String {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_string(),
        Err(e) => {
            println!("{}", e.to_string().red());
            "".to_string()
        }
    };
    current_dir
}
fn create_default_files( path: String ){
    let mut js_path = path.clone();
    let mut html_path = path.clone();

    js_path.push_str(JS_FILE);
    html_path.push_str(HTML_FILE);


    _ =  match File::create( js_path ) {
        Ok( mut file) => file.write_all(JS_CONTENT.as_bytes()),
        Err(e) =>{
            println!("{}", e.to_string().red());
            return;
        },
    };

    _ =  match File::create( html_path ) {
        Ok( mut file) => file.write_all(HTML_BOILERPLATE.as_bytes()),
        Err(e) =>{
            println!("{}", e.to_string().red());
            return;
        },
    };
}

async fn clone_repo( url: &str, path: String ){
    println!("{}", "cloning javascript repository...".blue());
    _ = match git2::Repository::clone(url, path.clone()) {
        Ok(_) => println!("{}","successfully cloned repository".green()),
        Err(e) => println!("{} {}", "error cloning repository".red().bold(), e.to_string().red().bold() ),
    };
}

pub async fn update_project(){
    let mut path = get_path();
    if path.len() <= 0 {
        println!("{}", "command failed".red());
        return;
    }
    let mut sticker_lib_path = String::from(path.clone());
    sticker_lib_path.push_str(LIB_DIR);

    if !std::fs::metadata(&sticker_lib_path).is_ok() {
        println!("{}", "failed to update javascript core, no directory found".red().bold());
        return;
    }
    _ = match std::fs::remove_dir_all(sticker_lib_path.clone()) {
        Ok(_) => println!("{}", "old js core directory removed successfully".green()),
        Err(e) => println!("{}", e.to_string().red().bold()),
    };

    clone_repo(REPO_URL, sticker_lib_path.clone()).await;
    
    sticker_lib_path.push_str(CONFIG_FILE);

    path.push_str(CONFIG_FILE);

    _ = match std::fs::remove_file( sticker_lib_path){
        Ok(_)=> println!("{}","config file removed successfully".green()),
        Err(e) => println!("{}", e.to_string().red().bold()),
    };
    println!("{}", " Update finished!".green());
}

pub async fn create_new_project() {
    let mut path = get_path();
    if path.len() <= 0 {
        println!("{}", "command failed".red());
        return;
    }
    let mut sticker_lib_path = String::from(path.clone());
    sticker_lib_path.push_str(LIB_DIR);
    clone_repo(REPO_URL, sticker_lib_path.clone()).await;

    sticker_lib_path.push_str(CONFIG_FILE);
    create_default_files(path.clone());

    path.push_str(CONFIG_FILE);

    _ = match std::fs::rename( sticker_lib_path, path){
        Ok(_)=> println!("{}","config file moved successfully".green()),
        Err(e) => println!("{}", e.to_string().red().bold()),
    };
    println!("{}", "Finished!".green());
}
