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
pub struct RawCardSet {
    set_name: String,
    set_code: String,
    num_of_cards: u32,
    tcg_date: Option<String>,
}

impl RawCardSet {
    pub fn convert(&self) -> CardSet {
        CardSet {
            name: self.set_name.clone(),
            code: self.set_code.clone(),
            num_cards: self.num_of_cards,
            tcg_date: None, // TODO: parse this
        }
    }
}
