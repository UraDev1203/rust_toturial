use std::{env, process};
use rust_toturial::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguements: {err}");
        process::exit(1);
    });

    if let Err(e) = rust_toturial::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}




