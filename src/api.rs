extern crate chrono;

use chrono::offset::Utc;
use chrono::Date;

struct CardSet {
    name: String,
    code: String,
    num_cards: u32,
    tcg_date: Date<Utc>,
}

trait API {
    fn get_cardsets(&mut self) -> Result<Vec<CardSet>, String>;
}

struct YGOProDeckAPI {
    config: APIConfig,
}

impl YGOProDeckAPI {
    fn new(config: APIConfig) -> YGOProDeckAPI {
        YGOProDeckAPI { config }
    }
}

impl API for YGOProDeckAPI {
    fn get_cardsets(&mut self) -> Result<Vec<CardSet>, String> {
        let request_url = format!("{}/cardsets.php", self.config.api_path);

        panic!()
    }
}

struct APIConfig {
    api_path: String,
}

impl APIConfig {
    fn new(api_path: &str) -> APIConfig {
        APIConfig {
            api_path: api_path.to_string(),
        }
    }
}
