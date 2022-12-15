use std::collections::HashMap;
use serde_derive::{Deserialize};
use toml::Value;

#[derive(Deserialize)]
pub struct Config {
    pub input: HashMap<String, String>,
    pub behaviour: HashMap<String, Value>,
    pub rules: HashMap<String, String>,
}