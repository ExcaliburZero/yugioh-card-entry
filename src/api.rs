extern crate chrono;
extern crate reqwest;
extern crate serde;

use crate::data::*;

pub trait API {
    fn get_cardsets(&mut self) -> Result<Vec<CardSet>, String>;
}

pub struct YGOProDeckAPI {
    config: APIConfig,
}

impl YGOProDeckAPI {
    pub fn new(config: APIConfig) -> YGOProDeckAPI {
        YGOProDeckAPI { config }
    }
}

impl API for YGOProDeckAPI {
    fn get_cardsets(&mut self) -> Result<Vec<CardSet>, String> {
        let request_url = format!("{}/cardsets.php", self.config.api_path);

        match reqwest::blocking::get(&request_url) {
            Ok(response) => Ok(response.json::<Vec<CardSet>>().unwrap()),
            Err(error) => Err(error.to_string()),
        }
    }
}

pub struct APIConfig {
    api_path: String,
}

impl APIConfig {
    pub fn new(api_path: &str) -> APIConfig {
        APIConfig {
            api_path: api_path.to_string(),
        }
    }
}
