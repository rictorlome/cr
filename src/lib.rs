pub mod rules;

use crate::rules::Rules;

use anyhow::Result;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

pub fn read_replace_write<R: Read, W: Write>(read: R, write: W, rules: &Rules) -> Result<()> {
    let mut line = String::new();
    let mut reader = BufReader::new(read);
    let mut writer = BufWriter::new(write);
    loop {
        match reader.read_line(&mut line)? {
            0 => {
                writer.flush()?;
                return Ok(());
            }
            _ => {
                let new_line = replace_in_line(&mut line, rules);
                writer.write_all(new_line.as_bytes())?;
                line.clear();
            }
        }
    }
}

pub fn replace_in_line(line: &String, rules: &Rules) -> String {
    rules.iter().fold(String::from(line), |acc, (old, new)| {
        if acc.contains(old) {
            acc.replace(old, new)
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn replace_in_line_works() {
        let mut line1 = "ralph".to_string();
        let mut line2 = "Ralph".to_string();
        let mut line3 = "RALPH".to_string();
        let rules = crate::rules::generate_rules(&"ralph".to_string(), &"tony".to_string());
        assert_eq!(super::replace_in_line(&mut line1, &rules), "tony");
        assert_eq!(super::replace_in_line(&mut line2, &rules), "Tony");
        assert_eq!(super::replace_in_line(&mut line3, &rules), "TONY");
    }
    #[test]
    fn replace_in_line_works_over_multiple_occurences() {
        let mut line = "ralph Ralph RALPH".to_string();
        let rules = crate::rules::generate_rules(&"ralph".to_string(), &"tony".to_string());
        assert_eq!(super::replace_in_line(&mut line, &rules), "tony Tony TONY");
    }
    #[test]
    fn replace_in_line_ignores_irrelevant_names() {
        let mut line = "bob Bob BOB".to_string();
        let rules = crate::rules::generate_rules(&"ralph".to_string(), &"tony".to_string());
        assert_eq!(super::replace_in_line(&mut line, &rules), line);
    }
    #[test]
    fn read_replace_write_works() -> anyhow::Result<()> {
        let read = String::from("Ralph\nralph");
        let mut write: Vec<u8> = Vec::new();
        let rules = crate::rules::generate_rules(&"ralph".to_string(), &"tony".to_string());
        super::read_replace_write(read.as_bytes(), &mut write, &rules)?;
        assert_eq!(write, "Tony\ntony".as_bytes());
        Ok(())
    }
}
