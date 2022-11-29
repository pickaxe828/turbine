use std::fs;
use toml;

mod config_parse;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        println!("omg")
    }

    let config_raw = fs::read_to_string("./config.turbine.toml").unwrap();
    let config: config_parse::Config = toml::from_str(&config_raw).unwrap();

    let instructions = config.rules.iter().map(|(k, v)| {
        println!("{}: {}", k, v);
    });

    let input = String::from(config.input["input"].as_str().unwrap());
    for i in String::from(input).chars() {
        println!("{:?}", i);
    }
}