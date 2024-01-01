mod cli;
#[tokio::main] 
async fn main() {
    cli::get_command().await;
}
