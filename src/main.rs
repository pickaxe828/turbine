use std::{fs, collections::HashMap};
use toml::{self, Value};
use std::{thread, time::Duration};

mod config_parse;

fn text_to_squares(input: &String) -> String {
    input.replace("0", "  ").replace("1", "██")
}

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        // preserved for future use
        println!("Arguments detected.")
    }

    // Read config file
    let config_raw = fs::read_to_string("./config.turbine.toml").unwrap();
    let config = &config_raw.parse::<Value>().unwrap();

    let mut input = String::from(config["input"]["tape"].as_str().unwrap());
    let behaviour = config["behaviour"].as_table().unwrap();
    let rules = config["rules"].as_table().unwrap();

    let reader_width = behaviour.get("reader_width").unwrap().as_integer().unwrap() as usize;
    
    // check if all the tape, rules and the reader is valid
    if reader_width > input.chars().count() {
        panic!("Reader width is larger than the tape length.");
    }

    // how stuff begins
    // **Mechanical Squeaking noise**
    println!("{}", text_to_squares(&input));
    loop {
        let mut result: Vec<String> = Vec::new();
        input.chars().into_iter().enumerate().for_each(|(ind, val)| {
            if ind < (input.chars().count() - reader_width + 1) {
                result.push(rules.get(&input.get(ind..ind + reader_width).unwrap().to_string()).unwrap().to_string());
            }
        });
        let output = format!("0{}0", result.concat());
        println!("{}", text_to_squares(&output));
        input = output;
        thread::sleep(Duration::from_millis(100));
    }
}