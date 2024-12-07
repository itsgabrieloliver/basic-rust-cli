use basic_cli::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Failed to generate configuration: {}", error);
        process::exit(1);
    });

    if let Err(error) = config.run() {
        eprintln!("Application error: {error}");
        process::exit(1);
    };
}
