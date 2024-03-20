mod commands;
//mod handler;

use commands::Args;
fn main() {
let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}