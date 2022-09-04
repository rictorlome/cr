pub mod config;

use crate::config::Config;

use anyhow::{anyhow, Result};
use std::fs::{rename, File};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use tempfile::tempdir;

pub type Rules = Vec<(String, String)>;

pub fn replace(cfg: &Config) -> Result<()> {
    let rules = generate_rules(&cfg.from, &cfg.to);
    let mut files = cfg.files.clone();
    files.dedup();
    println!("Applying the following rules (old, new): {:?}", rules);
    for file in files {
        replace_in_file(&file, &rules)?
    }
    Ok(())
}

pub fn generate_rules(from: &String, to: &String) -> Rules {
    vec![
        // Unmodified
        (from.clone(), to.clone()),
        // Lowercase
        (from.to_lowercase(), to.to_lowercase()),
        // Uppercase
        (from.to_uppercase(), to.to_uppercase()),
        // Capitalized
        (capitalize_first_letter(&from), capitalize_first_letter(&to)),
    ]
}

// Stolen from https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
pub fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn replace_in_file(path: &PathBuf, rules: &Rules) -> Result<()> {
    let f = File::open(&path)?;
    let tmpdir = tempdir()?;
    let tmpfile_path = tmpdir.path().join(&path);
    let tmpfile: File = File::create(&tmpfile_path)?;

    let mut reader = BufReader::new(f);
    let mut line = String::new();
    // Read from old, write to new
    {
        let mut writer = BufWriter::new(tmpfile);
        loop {
            match reader.read_line(&mut line) {
                Ok(0) => {
                    writer.flush()?;
                    break;
                }
                Ok(_) => {
                    let new_line = replace_in_line(&mut line, rules);
                    writer.write_all(new_line.as_bytes())?;
                    line.clear();
                }
                Err(e) => return Err(anyhow!(e.to_string())),
            }
        }
    }
    // Replace
    rename(&tmpfile_path, &path)?;
    tmpdir.close()?;
    Ok(())
}

pub fn replace_in_line(s: &String, rules: &Rules) -> String {
    let mut new_line = String::from(s);
    for (old, new) in rules {
        new_line = new_line.replace(old, new);
    }
    new_line
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_rules_works() {
        let rules = super::generate_rules(&"ralph".to_string(), &"tony".to_string());
        assert!(rules.contains(&("ralph".to_string(), "tony".to_string())));
        assert!(rules.contains(&("RALPH".to_string(), "TONY".to_string())));
        assert!(rules.contains(&("Ralph".to_string(), "Tony".to_string())));
    }

    #[test]
    fn replace_in_line_works() {
        let mut line1 = "ralph".to_string();
        let mut line2 = "Ralph".to_string();
        let mut line3 = "RALPH".to_string();
        let rules = super::generate_rules(&"ralph".to_string(), &"tony".to_string());
        assert_eq!(super::replace_in_line(&mut line1, &rules), "tony");
        assert_eq!(super::replace_in_line(&mut line2, &rules), "Tony");
        assert_eq!(super::replace_in_line(&mut line3, &rules), "TONY");
    }
}
