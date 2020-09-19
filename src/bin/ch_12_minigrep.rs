// similar to env::args, but accepts CL arguments that are not valid unicode, like maybe piped in
// binary
// use std::env::args_os;
use lib::minigrep;
use std::env;

fn main() {
    // invoking the collect implemented specifically for T: Vec<String>
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        std::process::exit(1);
    });

    println!(
        "\nSearching for: {:?}\nIn File: {:?}\n",
        config.query, config.filename
    );

    // read_to_string() returns a Result<String, std::error::Error>

    // ownership is not taken by println! even though we are passing a non-reference

    // ownership is taken of config.filename here

    if let Err(e) = minigrep::run(config) {
        println!("App error: {}", e);
        std::process::exit(1);
    }
}
