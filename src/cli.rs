use std::env;
use crate::{run, cs};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    
    match command {
        "go"     => _ = run::main(),
        "server" => cs::server(),
        "client" => cs::client(),
        _        => println!("Unknown command: {}", command),
    };
}
