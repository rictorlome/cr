use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "cr")]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(long, value_parser, required = true)]
    pub files: Vec<PathBuf>,
    #[clap(short, long, value_parser)]
    pub from: String,
    #[clap(short, long, value_parser)]
    pub to: String,
}
