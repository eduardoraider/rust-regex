use regex::Regex;

fn main() {

    let text = "Hello, World!";

    // Simple matching
    let pattern = Regex::new(r"Hello").unwrap();
    if pattern.is_match(text) {
        println!("Found a match for 'Hello'");
    }

    // Capturing groups
    let pattern_with_group = Regex::new(r"(\w+), (\w+)").unwrap();
    if let Some(captures) = pattern_with_group.captures(text) {
        println!("Captured groups: {}, {}", &captures[1], &captures[2]);
    }

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        println!("Did our date match? {}", re.is_match("2014-01-01"));

    // Replacement
    let replaced_text = pattern_with_group.replace_all(text, "$2, $1");
    println!("Replaced text: {}", replaced_text);
}