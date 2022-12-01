use std::collections::HashMap;
use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub input: HashMap<String, String>,
    pub behaviour: HashMap<String, String>,
    pub rules: HashMap<String, String>,
}