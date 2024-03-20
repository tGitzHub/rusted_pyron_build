mod commands;
mod packages;
mod utils;

use std::env;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Handle commands
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "install" => println!("Installing nothing"),//commands::install(&args), // Handle "install" command
            _ => println!("Unknown command: {}", command),
        }
    } else {
        println!("Usage: rustedpyron <command>");
    }
}
