use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "cr")]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional path of file to read in. If none supplied, will read from stdin
    #[clap(short, long, value_parser, required = false)]
    pub path: Option<PathBuf>,
    /// The old name
    #[clap(short, long, value_parser)]
    pub from: String,
    /// The new name
    #[clap(short, long, value_parser)]
    pub to: String,
}
