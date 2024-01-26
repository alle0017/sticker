mod cli;
use std::time::SystemTime;

#[tokio::main] 
async fn main() {
    let sys_time = SystemTime::now();
    cli::get_command().await;
    println!("program ended in {} seconds", SystemTime::now().duration_since(sys_time).unwrap().as_secs_f32());
}
