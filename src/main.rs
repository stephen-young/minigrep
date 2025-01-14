use std::env::args;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Looking for {} in {}", config.query, config.file_name);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
