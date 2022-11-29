use toml;
use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub input: toml::map::Map<String, toml::value::Value>,
    pub behaviour: toml::map::Map<String, toml::value::Value>,
    pub rules: toml::map::Map<String, toml::value::Value>,
}