use std::{env, process};

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!(
        "in file {} {}\n",
        config.file_path,
        if config.ignore_case { "(case insensitive)" } else { "" }
    );

    if let Err(error) = run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
