use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with reading args: {}", err);
        process::exit(1);
    });

    println!("we are looking for {:?}", config.query);
    println!("in the file with the name {:?}", config.filename);
    if let Err(e) = minigrep::run(&config) {
        println!("application error {}", e);

        process::exit(1);
    }
}

