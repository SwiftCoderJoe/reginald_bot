use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,

    pub annoyed_person: u64,

    pub true_gif: String,
    pub false_gif: String,
    pub perhaps_gif: String,
}

pub fn read_config() -> Config {
    let mut input = String::new();

    File::open("config.toml")
        .and_then(|mut f| f.read_to_string(&mut input))
        .unwrap();

    toml::from_str(&input).unwrap()
}