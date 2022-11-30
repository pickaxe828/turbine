use std::{fs};
use toml;
mod config_parse;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        // preserved for future use
        println!("Arguments detected.")
    }

    // Read config file
    let config_raw = fs::read_to_string("./config.turbine.toml").unwrap();
    let config: config_parse::Config = toml::from_str(&config_raw).unwrap();

    let rules = config.rules;

    let input = format!("00{}00", String::from(rules["input"].as_str().unwrap()));
    let mut output: Vec<&String> = Vec::new();
    for i in 0..input.len()-2 {
        let instruction = &input[i..i+3];
        let out = rules[instruction].to_string();
        output.push(&out);
    }
}