use rocket::serde::{Serialize, Deserialize, json};
use crate::models::cards::CardEntity;

#[derive(Serialize, Deserialize)]
pub struct CardSerialized {
    attributes: Vec<json::Value>,
    description: String,
    external_url: String,
    image: String,
    name: String
}

impl CardSerialized {
    pub fn get_by_pk(token_id: u8) -> CardSerialized {
        info!("token_id: {}", token_id);
        let card = CardEntity::get_by_pk(token_id);
        CardSerialized {
            attributes: vec!(),
            description: card.description,
            external_url: card.external_url,
            image: card.image,
            name: card.name
        }
    }
}