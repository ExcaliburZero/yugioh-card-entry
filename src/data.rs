extern crate chrono;
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub desc: String,
    pub atk: u32,
    pub def: u32,
    pub level: u32,
    pub race: String,
    pub attribute: String,
    pub card_sets: Vec<CardSetEntry>,
    pub card_images: Vec<CardImage>,
    pub card_prices: Vec<CardPrices>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CardSetEntry {
    pub set_name: String,
    pub set_code: String,
    pub set_rarity: String,
    pub set_rarity_code: String,
    pub set_price: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CardImage {
    pub id: u64,
    pub image_url: String,
    pub image_url_small: String,
}

pub type CardPrices = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CardInfoResponse {
    pub data: Vec<Card>,
}
