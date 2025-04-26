use std::env;
use crate::{run, cs, app};

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    
    match command {
        "go"     => _ = run::main(),
        "server" => cs::server(),
        "client" => cs::client(),
        "app"    => app::client::Client().await,
        _        => println!("Unknown command: {}", command),
    };
}
