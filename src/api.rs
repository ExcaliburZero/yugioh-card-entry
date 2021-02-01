extern crate chrono;
extern crate reqwest;
extern crate serde;

use crate::data::*;

pub trait API {
    fn get_cardsets(&mut self) -> Result<Vec<CardSet>, String>;
    fn get_cardinfo(&mut self, request: &CardInfoRequest) -> Result<Vec<Card>, String>;
}

pub struct CardInfoRequest {
    pub name: String,
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

    fn get_cardinfo(&mut self, request: &CardInfoRequest) -> Result<Vec<Card>, String> {
        let request_url = format!("{}/cardinfo.php", self.config.api_path);

        let client = reqwest::blocking::Client::new();
        match client
            .get(&request_url)
            .query(&[("name", &request.name)])
            .send()
        {
            Ok(response) => Ok(response.json::<CardInfoResponse>().unwrap().data),
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
