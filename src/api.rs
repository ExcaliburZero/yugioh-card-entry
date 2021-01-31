extern crate chrono;
extern crate reqwest;
extern crate serde;

use chrono::offset::Utc;
use chrono::Date;
use serde::{Deserialize, Serialize};

pub struct CardSet {
    pub name: String,
    pub code: String,
    pub num_cards: u32,
    pub tcg_date: Option<Date<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawCardSet {
    set_name: String,
    set_code: String,
    num_of_cards: u32,
    tcg_date: Option<String>,
}

impl RawCardSet {
    fn convert(&self) -> CardSet {
        CardSet {
            name: self.set_name.clone(),
            code: self.set_code.clone(),
            num_cards: self.num_of_cards,
            tcg_date: None, // TODO: parse this
        }
    }
}

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
            Ok(response) => {
                let raw_cardsets = response.json::<Vec<RawCardSet>>().unwrap();

                Ok(raw_cardsets.iter().map(|r| r.convert()).collect())
            }
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
