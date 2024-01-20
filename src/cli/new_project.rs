extern crate colored;
extern crate git2;

use colored::Colorize;

const REPO_URL: &str = "https://github.com/alle0017/stickerJS";

fn get_path( name: String ) -> String {
    let mut current_dir = match std::env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_string(),
        Err(e) => {
            println!("{}", e.to_string().red());
            "".to_string()
        }
    };
    current_dir.push('/');
    current_dir.push_str(name.as_str() );
    current_dir
}

async fn clone_repo( url: &str, path: String ){
    println!("{}", "cloning javascript repository...".green());
    _ = match git2::Repository::clone(url, path) {
        Ok(_) => println!("{}","successfully cloned repository".green()),
        Err(e) => println!("{} {}", "ERROR CLONING REPOSITORY".red().bold(), e ),
    };
}

pub async fn create_new_project( dir_name: String ) {
    let path = get_path( dir_name );
    clone_repo(REPO_URL, path).await;
    println!("{}", "Finished!".green());
}
