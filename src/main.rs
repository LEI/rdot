use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;

mod cli;
mod config;

// Action: install/remove
// Directory: source -> target
// Strategy: local, ansible

fn main() {
    let args = Cli::parse();
    let config = Config::load()?;

    println!("Hello, world! Args: {:#?}", args);

    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
