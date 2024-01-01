extern crate colored;

mod dom;
mod config;
mod new_project;

use dom::Dom;
use dom::CustomTagParser;


use colored::Colorize;

use std::env;


const  BUILD_COMMAND: &str = "comp";
const BUILD_FROM_CONFIG: &str = "build";
const NEW_PROJECT_COMMAND: &str = "new";


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
            println!("{} {}", "working on ".green().bold().on_white(), target.i.green().bold().on_white() );
            let mut dom: Dom = Dom::new(&target.i);
            dom.parse();
            dom.create_file(&target.o);
            println!("{} {} {} {}", "file".green().bold().on_white(), target.i.green().bold().on_white(), "printed in".green().bold().on_white(),  target.o.green().bold().on_white() );

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
      
      println!("file path acquired, {}", file_path.green());
      //let dom = create_html_dom(file_path);
      let mut dom: Dom = Dom::new(&file_path);
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
      println!("{} {}", "command".magenta(), "args".green());
      println!("sticker {} {}", BUILD_COMMAND.magenta(), " [file_to_compile] [compiled_result_file_name]".bold().green());
}
pub fn get_command(){
      let args: Vec<String> = env::args().collect();
      if args.len() <= 1 {
            println!("no arguments found. run {} to see full command list", "sticker".blue() );
            return;
      }
      if args[1] == BUILD_COMMAND.to_string() {
            build_single_file(2);
      } else if args[1] == NEW_PROJECT_COMMAND.to_string() {
            
      }else if args[1] == BUILD_FROM_CONFIG.to_string() {
            build_from_config();
      }else {
            print_command_list();
      }
}