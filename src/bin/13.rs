use core::panic;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, String> {
    let mut profile = HashMap::new();
    for pair in input.split('&') {
        let (key, value) = pair.split_once('=').unwrap();
        profile.insert(key.to_string(), value.to_string());
    }
    profile
}

fn profile_for(input: &str) -> String {
    if input.contains('=') || input.contains('&') {
        panic!("making code less cancer");
    }
    format!("email={}&uid=10&role=user", input)
}

fn main() {
    todo!();
}
