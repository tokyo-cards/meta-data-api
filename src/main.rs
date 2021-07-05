#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json};

#[derive(Serialize, Deserialize)]
struct Card {
    attributes: Vec<json::Value>,
    description: String,
    external_url: String,
    image: String,
    name: String
}

#[get("/api/cards/<token_id>")]
fn cards(token_id: u8) -> json::Json<Card> {
    let data = r#"
    {
        "attributes": [
            {
                "trait_type": "Deck",
                "value": 0
            },
            {
                "display_type": "number",
                "trait_type": "Deck",
                "value": 0
            },
            {
                "trait_type": "DeckName",
                "value": "MNEMONICS at A Bar"
            },
            {
                "trait_type": "Sign",
                "value": "A"
            },
            {
                "trait_type": "Suite",
                "value": "Joker"
            },
            {
                "trait_type": "Kana",
                "value": "つ"
            },
            {
                "trait_type": "Mnemonic",
                "value": "acquire"
            },
            {
                "trait_type": "Rarity",
                "value": "Epic"
            }
        ],
        "description": "Trading Poker Cards from Diva.Cards",
        "external_url": "",
        "image": "https://diva.cards/images/card_set/anderson/0/ja.png",
        "name": "acquire.joker.a"
    }"#;
    let c: Card = json::from_str(data).unwrap();
    json::Json(c)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cards])
}
