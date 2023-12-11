
extern crate colored;

mod dom;

use dom::Dom;
use dom::CustomTagParser;


use colored::Colorize;

use std::env;


fn get_file_path() -> String {
    let mut file_path: String = String::new();
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            println!("{}", e.to_string().red());
            panic!();
        },
    };

    println!("{}", current_dir.to_str().unwrap().bold().on_white().black());
    println!("{}","insert file path from current directory:".bold().on_white().black());
    std::io::stdin().read_line(&mut file_path).unwrap();
    file_path = file_path.replace("\n", "");
    file_path.insert(0, '/');
    file_path.insert_str(0, current_dir.to_str().unwrap());
    file_path
}
fn main() {

    let mut file_path: String;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        file_path = args[1].to_string();
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("{}", e.to_string().red());
                panic!();
            },
        };
        file_path.insert(0, '/');
        file_path.insert_str(0, current_dir.to_str().unwrap());
    } else {
        file_path = get_file_path();
    }
    
    println!("file path acquired, {}", file_path.green());
    //let dom = create_html_dom(file_path);
    let mut dom: Dom = Dom::new(&file_path);
    dom.parse();

    if args.len() > 2 {
        file_path = args[2].to_string();
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("{}", e.to_string().red());
                panic!();
            },
        };
        file_path.insert(0, '/');
        file_path.insert_str(0, current_dir.to_str().unwrap());
    } else {
        file_path = get_file_path();
    }

    dom.create_file(&file_path);
}
