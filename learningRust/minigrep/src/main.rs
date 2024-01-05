use std::env;
use std::fs;

mod minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grep = minigrep::build(args).unwrap();

    minigrep::run(grep).unwrap();
}
