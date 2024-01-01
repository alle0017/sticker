extern crate colored;
extern crate reqwest;

use colored::Colorize;
use std::{fs::File, io::Write};

const JS_FILE: &str = "sticker.js";
const CONFIG_FILE: &str = "sticker-config.json";
const JS_URL: &str = "https://raw.githubusercontent.com/alle0017/sticker/master/sticker.js";
const CONFIG_URL: &str = "https://raw.githubusercontent.com/alle0017/sticker/master/sticker-config.json";

fn get_path(name: &str) -> String {
    let mut current_dir = match std::env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_string(),
        Err(e) => {
            println!("{}", e.to_string().red());
            "".to_string()
        }
    };
    current_dir.push('/');
    current_dir.push_str(name);
    current_dir
}

async fn fetch_data(url: &str) -> Option<String> {
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(err) => {
            println!(
                "{} {}",
                "failed to fetch data".red().bold(),
                err.to_string().red().bold()
            );
            return None;
        }
    };
    if !response.status().is_success() {
        println!("{}", "failed to fetch data".red().bold());
        return None;
    }
    let text = match response.bytes().await {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => return None,
    };
    Some(text)
}

async fn create_file_from_url(url: &str, file_name: &str) {
    let text = match fetch_data(url).await {
        Some(text) => text,
        None => return,
    };
    let path = get_path(file_name);
    println!("{}", path.yellow());
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "{} {}",
                "Error creating file".red().bold(),
                e.to_string().red().bold()
            );
            return;
        }
    };
    if let Err(e) = file.write_all(text.as_bytes()) {
        println!(
            "{} {}",
            "Error writing to file".red().bold(),
            e.to_string().red().bold()
        );
    }
}

pub async fn create_new_project() {
      println!("{}", "Creating sticker.js file...".green());
      create_file_from_url(JS_URL, JS_FILE).await;
      println!("{}", "Creating sticker-config.json file...".green());
      create_file_from_url(CONFIG_URL, CONFIG_FILE).await;
      println!("{}", "Finished!".green());
}
