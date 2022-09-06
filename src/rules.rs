pub type Rules = Vec<(String, String)>;

pub fn generate_rules(from: &String, to: &String) -> Rules {
    vec![
        // Unmodified
        (from.clone(), to.clone()),
        // Lowercase
        (from.to_lowercase(), to.to_lowercase()),
        // Uppercase
        (from.to_uppercase(), to.to_uppercase()),
        // Capitalized
        (
            capitalize_first_letter(&from.to_lowercase()),
            capitalize_first_letter(&to.to_lowercase()),
        ),
    ]
}

// Stolen from https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_rules_works() {
        let rules = super::generate_rules(&"ralpH".to_string(), &"tonY".to_string());
        assert!(rules.contains(&("ralpH".to_string(), "tonY".to_string())));
        assert!(rules.contains(&("ralph".to_string(), "tony".to_string())));
        assert!(rules.contains(&("RALPH".to_string(), "TONY".to_string())));
        assert!(rules.contains(&("Ralph".to_string(), "Tony".to_string())));
    }
}
