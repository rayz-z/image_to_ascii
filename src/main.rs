use ::std::env;
use image_to_ascii::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = image_to_ascii::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
