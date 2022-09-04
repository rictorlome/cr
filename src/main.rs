use anyhow::Result;

use clap::Parser;
use cr::config::Config;

fn main() -> Result<()> {
    let args = Config::parse();
    println!("Args: {:?}", args);
    match cr::replace(&args) {
        Ok(_) => {
            println!("Success!");
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}
