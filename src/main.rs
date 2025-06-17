use std::{env, process};
use rust_toturial::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguements: {err}");
        process::exit(1);
    });

    if let Err(e) = rust_toturial::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}




