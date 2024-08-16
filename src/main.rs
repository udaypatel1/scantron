mod args;
mod utils;
use args::Args;
use utils::execute;
use std::fs;

fn main() {
    let args = Args::new();

    match args.quiet {
        true => {
            let json = execute(&args.path);
            fs::write("scantron.json", json.as_bytes()).unwrap();
        }
        false => {
            let json = execute(&args.path);
            println!("{}", json);
        }
    }
}