extern crate colored;
extern crate serde_json;
extern crate serde_derive;

use serde_derive::Deserialize;

use colored::Colorize;
use serde_json::from_str;
use std::{fs::File, io::Read, path::Path};

const CONFIG_FILE: &str = "sticker-config.json";

#[derive(Deserialize)]
pub struct CompileTarget {
      pub i: String,
      pub o: String,
}
#[derive(Default,Deserialize)]
struct CompileOptions {
      targets: Vec<CompileTarget>,
      compDir: Option<String>,
}

fn get_config_file_path() -> String {
      let mut current_dir = match std::env::current_dir() {
            Ok(dir) => dir.to_str().unwrap().to_string(),
            Err(e) => {
                println!("{}", e.to_string().red());
                "".to_string()
            },
      };
      current_dir.push('/');
      current_dir.push_str(CONFIG_FILE);
      println!( "expected file path: {}", current_dir.yellow() );
      current_dir
}

fn get_config( path: String ) -> CompileOptions {
      let mut content: String = String::new();
      let _ = match File::open(path) {
            Ok(mut f) => f.read_to_string(&mut content),
            Err(e) =>{ 
                  println!("{} {}", "error while opening config file".red(), e.to_string().red() );
                  return CompileOptions::default();
            },
      };
      let json: CompileOptions = match from_str(&content) {
            Ok(json) => json,
            Err(_) => {
                  println!("{}", "error while parsing config file".red().bold() );
                  return CompileOptions::default();
            }
      } ;
      json
}
fn get_compile_dir( json: &CompileOptions ) -> String {
      let dir = match &json.compDir {
            Some(file) => file,
            None => {
                  let current_dir = match std::env::current_dir() {
                        Ok(dir) => dir,
                        Err(e) => {
                            println!("{}", e.to_string().red());
                            panic!();
                        },
                  };
                  let mut path = current_dir.to_str().unwrap().to_string();
                  path.push('/');
                  return path;
            }
      };
      let mut path = Path::new(&dir).canonicalize().unwrap().to_str().unwrap().to_string();
      if !path.ends_with('/') {
            path.push('/');
      }
      path
}

pub fn get_compile_targets() -> Vec<CompileTarget> {
      let path = get_config_file_path();

      let mut res = Vec::new();

      if path.len() == 0 {
            println!("{}", "no config file found".red().bold());
            return res;
      }
      let json: CompileOptions = get_config(path);
      let dir = get_compile_dir(&json);
      let current_dir = match std::env::current_dir() {
            Ok(dir) => {
                                    let mut path = dir
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                    path.push('/');
                                    path
                        },
            Err(e) => {
                println!("{}", e.to_string().red());
                panic!();
            },
      };
      for mut target in json.targets {
            target.i.insert_str(0, current_dir.as_str());
            target.o.insert_str(0, dir.as_str());
            res.push( CompileTarget {
                  i: target.i,
                  o: target.o,
            })
      }
      res
}


