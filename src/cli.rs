extern crate colored;

mod dom;
mod config;
mod new_project;
mod data;
mod server;

use dom::Dom;
use dom::CustomTagParser;


use colored::Colorize;

use std::env;


const BUILD_COMMAND: &str = "comp";
const BUILD_FROM_CONFIG: &str = "build";
const NEW_PROJECT_COMMAND: &str = "new";
const UPDATE: &str = "update";
const SERVE: &str = "serve";

const VERSION: &'static str = "v 0.1.0";

/**
 * ask for input the relative file path to the i/o file
 */
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
fn build_from_config(){
      let targets = config::get_compile_targets();
      for target in targets {
            println!("working on {}...", target.i.blue().bold() );
            let mut dom: Dom = Dom::new(&target.i);
            data::get_data_from_file(&mut dom.dom);

            dom.parse();
            dom.create_file(&target.o);
            println!("input file: {} output file: {}", target.i.blue().bold(), target.o.blue().bold() );
      }
}
fn build_single_file( base_arg: usize ){
      let mut file_path: String;
      let args: Vec<String> = env::args().collect();
      if args.len() > base_arg {
            file_path = args[base_arg].to_string();
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
      
      println!("file path acquired, {}...", file_path.blue().bold());
      //let dom = create_html_dom(file_path);
      let mut dom: Dom = Dom::new(&file_path);
      data::get_data_from_file(&mut dom.dom);
      dom.parse();

      if args.len() > base_arg + 1 {
            file_path = args[base_arg + 1].to_string();
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

fn print_command_list(){
      println!( "\n{} {}\n", "\t STICKER CLI ૮꒰ ˶• ༝ •˶꒱ა ".bold().bright_blue().on_white(), VERSION );
      println!("   sticker {} {}\n", "command".magenta(), "[args]".green());
      println!("{} {} {}", "sticker".bold(), BUILD_COMMAND.magenta(), " [file_to_compile] [compiled_result_file_name]".bold().green());
      println!("{} {}", "sticker".bold(), BUILD_FROM_CONFIG.magenta());
      println!("{} {}", "sticker".bold(), NEW_PROJECT_COMMAND.magenta());
      println!("{} {}", "sticker".bold(), UPDATE.magenta());
      println!("{} {}", "sticker".bold(), SERVE.magenta());
}
#[allow(unused_assignments)]
pub async fn get_command(){
      let args: Vec<String> = env::args().collect();
      if args.len() <= 1 {
            print_command_list();
            return;
      }
      if args[1] == BUILD_COMMAND.to_string() {
            build_single_file(2);
      } else if args[1] == NEW_PROJECT_COMMAND.to_string() {
            new_project::create_new_project().await;
      } else if args[1] == BUILD_FROM_CONFIG.to_string() {
            build_from_config();
      } else if args[1] == UPDATE.to_string() {
            new_project::update_project().await;
      } else if args[1] == SERVE.to_string(){
            server::serve().await;
      } else {
            print_command_list();
      }
}
