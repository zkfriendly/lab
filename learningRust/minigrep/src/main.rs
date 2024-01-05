use std::env;
use std::fs;
use std::process;

mod minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grep = minigrep::build(args).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    minigrep::run(grep).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
}
