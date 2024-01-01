extern crate colored;
//extern crate tokio;
extern crate reqwest;

use colored::Colorize;
use std::{fs::File, io::Write};

const JS_FILE: &str = "sticker.js";
const CONFIG_FILE: &str = "sticker-config.json";
const JS_URL: &str = "https://github.com/alle0017/sticker/blob/master/sticker.js";
const CONFIG_URL: &str = "https://github.com/alle0017/sticker/blob/master/sticker-config.json";

fn get_path( name: &str ) -> String {
      let mut current_dir = match std::env::current_dir() {
            Ok(dir) => dir.to_str().unwrap().to_string(),
            Err(e) => {
                println!("{}", e.to_string().red());
                "".to_string()
            },
      };
      current_dir.push('/');
      current_dir.push_str(name);
      current_dir
}

#[tokio::main]
async fn fetch_data( url: &str ) -> Option<String> {
      let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(err) => {
                  println!("{} {}", "failed to fetch data".red().bold(), err.to_string().red().bold());
                  return None;
            }
      };
      if !response.status().is_success() {
            println!("{}", "failed to fetch data".red().bold() );
            return None;
      }
      let text = match response.bytes().await {
            Ok(bytes) => std::str::from_utf8(&bytes).unwrap().to_string(),
            Err(_) => return None,
      };
      Some(text)
}
#[tokio::main]
async fn create_file_from_url( url: &str, file_name: &str ){
      let text = match fetch_data( url ) {
            Some( text ) => text,
            None => return
      };
      let path = get_path( file_name );
      let mut file = match File::create( path ) {
            Ok( file ) => file,
            Err(e) =>{
                  println!("{} {}","Error creating file".red().bold(), e.to_string().red().bold());
                  return;
            }
      };
      file.write_all(text.as_bytes());
}
pub fn create_new_project(){
      create_file_from_url(JS_URL, JS_FILE);
      create_file_from_url(CONFIG_URL, CONFIG_FILE);
}
