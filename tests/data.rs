extern crate yugioh_card_entry;

use yugioh_card_entry::*;

#[test]
fn parse_card_set_entry() {
    let data = r#"{
        "set_name": "Battles of Legend: Relentless Revenge",
        "set_code": "BLRR-EN084",
        "set_rarity": "Secret Rare",
        "set_rarity_code": "(ScR)",
        "set_price": "4.08"
      }"#;

    let actual: CardSetEntry = serde_json::from_str(data).unwrap();
    let expected = CardSetEntry {
        set_name: "Battles of Legend: Relentless Revenge".to_string(),
        set_code: "BLRR-EN084".to_string(),
        set_rarity: "Secret Rare".to_string(),
        set_rarity_code: "(ScR)".to_string(),
        set_price: "4.08".to_string(),
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_card_image() {
    let data = r#"{
        "id": 6983839,
        "image_url": "https://storage.googleapis.com/ygoprodeck.com/pics/6983839.jpg",
        "image_url_small": "https://storage.googleapis.com/ygoprodeck.com/pics_small/6983839.jpg"
      }"#;

    let actual: CardImage = serde_json::from_str(data).unwrap();
    let expected = CardImage {
        id: 6983839,
        image_url: "https://storage.googleapis.com/ygoprodeck.com/pics/6983839.jpg".to_string(),
        image_url_small: "https://storage.googleapis.com/ygoprodeck.com/pics_small/6983839.jpg"
            .to_string(),
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_card_prices() {
    let data = r#"{
        "cardmarket_price": "0.42",
        "tcgplayer_price": "0.48",
        "ebay_price": "2.99",
        "amazon_price": "0.77",
        "coolstuffinc_price": "0.99"
      }"#;

    let actual: CardPrices = serde_json::from_str(data).unwrap();
    let expected: CardPrices = vec![
        ("cardmarket_price".to_string(), "0.42".to_string()),
        ("tcgplayer_price".to_string(), "0.48".to_string()),
        ("ebay_price".to_string(), "2.99".to_string()),
        ("amazon_price".to_string(), "0.77".to_string()),
        ("coolstuffinc_price".to_string(), "0.99".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(expected, actual);
}

#[test]
fn parse_card_info_response() {
    let data = r#"{
        "data": [
          {
            "id": 6983839,
            "name": "Tornado Dragon",
            "type": "XYZ Monster",
            "desc": "2 Level 4 monsters\nOnce per turn (Quick Effect): You can detach 1 material from this card, then target 1 Spell/Trap on the field; destroy it.",
            "atk": 2100,
            "def": 2000,
            "level": 4,
            "race": "Wyrm",
            "attribute": "WIND",
            "card_sets": [
              {
                "set_name": "Battles of Legend: Relentless Revenge",
                "set_code": "BLRR-EN084",
                "set_rarity": "Secret Rare",
                "set_rarity_code": "(ScR)",
                "set_price": "4.08"
              },
              {
                "set_name": "Duel Devastator",
                "set_code": "DUDE-EN019",
                "set_rarity": "Ultra Rare",
                "set_rarity_code": "(UR)",
                "set_price": "1.4"
              },
              {
                "set_name": "Maximum Crisis",
                "set_code": "MACR-EN081",
                "set_rarity": "Secret Rare",
                "set_rarity_code": "(ScR)",
                "set_price": "4.32"
              }
            ],
            "card_images": [
              {
                "id": 6983839,
                "image_url": "https://storage.googleapis.com/ygoprodeck.com/pics/6983839.jpg",
                "image_url_small": "https://storage.googleapis.com/ygoprodeck.com/pics_small/6983839.jpg"
              }
            ],
            "card_prices": [
              {
                "cardmarket_price": "0.42",
                "tcgplayer_price": "0.48",
                "ebay_price": "2.99",
                "amazon_price": "0.77",
                "coolstuffinc_price": "0.99"
              }
            ]
          }
        ]
      }"#;

    // Test that the parse does not fail
    let _actual: CardInfoResponse = serde_json::from_str(data).unwrap();

    // TODO: assert eq
}
