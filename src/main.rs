mod cli;
use cli::Cli;
use cr::rules::{generate_rules, Rules};

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

fn replace_to_stdout(path: &Option<PathBuf>, rules: &Rules) -> Result<()> {
    let write = std::io::stdout();
    match path {
        Some(path) => {
            let read = File::open(&path)?;
            cr::read_replace_write(read, write, rules)
        }
        None => {
            let read = std::io::stdin();
            cr::read_replace_write(read, write, rules)
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let rules = generate_rules(&args.from, &args.to);
    replace_to_stdout(&args.path, &rules)
}
