extern crate chrono;
extern crate reqwest;
extern crate serde;

use chrono::offset::Utc;
use chrono::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CardSet {
    pub set_name: String,
    pub set_code: String,
    pub num_of_cards: u32,
    pub tcg_date: Option<String>,
    // TODO: change to use:  pub tcg_date: Option<Date<Utc>>,
}
