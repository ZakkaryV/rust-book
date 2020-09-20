// similar to env::args, but accepts CL arguments that are not valid unicode, like maybe piped in
// binary
// use std::env::args_os;
use lib::minigrep;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("App error: {}", e);
        std::process::exit(1);
    }
}
