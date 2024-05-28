extern crate colored;
extern crate git2;

use std::path::Path;
use std::process::Command;

use colored::Colorize;

const REPO_URL: &str = "https://github.com/alle0017/server";
const LIB_DIR: &str = "/server";

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

async fn clone_repo( url: &str, path: String ){
    println!("{}", "cloning server repository...".blue());
    _ = match git2::Repository::clone(url, path.clone()) {
        Ok(_) => println!("{}","successfully cloned repository".green()),
        Err(e) => println!("{} {}", "error cloning repository".red().bold(), e.to_string().red().bold() ),
    };
}


async fn create_new_server( server_lib_path: &String ) {
    clone_repo(REPO_URL, server_lib_path.clone()).await;

}

pub async fn serve(){
    let path = get_path();
    if path.len() <= 0 {
        println!("{}", "command failed".red());
        return;
    }
    let mut server_lib_path = String::from(path.clone());
    server_lib_path.push_str(LIB_DIR);
    if Path::new(&server_lib_path).exists() {
        let _ = match Command::new("npm").args(["run", "b"]).current_dir(server_lib_path).spawn() {
            Err(e) => println!("{} {}","error while running the command: ".red(), e.to_string().red().bold() ),
            Ok(_) => println!("{}", "successfully started the server".green() )
        };
    }else{
        create_new_server(&server_lib_path).await;
        let _ = match Command::new("npm").arg("install").current_dir(server_lib_path.clone()).spawn() {
            Err(e) => println!("{} {}","error while installing all the npm packages: ".red(), e.to_string().red().bold() ),
            Ok(_) =>{ 
                println!("{}", "successfully installed the npm packages.".green() );
                println!("{}", "re run \"sticker serve\" to start the server".on_white().green().bold() );
            }
        };
    }
}
