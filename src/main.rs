use std::{fs, iter::Map, collections::HashMap};
use toml;
use serde::*;
mod config_parse;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        println!("omg")
    }

    let config_raw = fs::read_to_string("./config.turbine.toml").unwrap();
    let config: config_parse::Config = toml::from_str(&config_raw).unwrap();

    let instructions = config.rules;
    
    println!("{:?}", instructions);

    let input = String::from(config.input["input"].as_str().unwrap());
    for i in String::from(input).chars() {
        println!("{:?}", i);
    }
}