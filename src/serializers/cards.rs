use crate::models::cards::CardEntity;
use rocket::response::status::NotFound;
use rocket::serde::{json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CardSerialized {
    attributes: Vec<json::Value>,
    description: String,
    external_url: String,
    image: String,
    name: String,
}

pub type CardResultJSON = Result<json::Json<CardSerialized>, NotFound<String>>;

impl CardSerialized {
    pub fn get_by_pk(token_id: u8) -> CardResultJSON {
        info!("token_id: {}", token_id);
        match CardEntity::get_by_pk(token_id) {
            Ok(card) => {
                let c = CardSerialized {
                    attributes: vec![],
                    description: card.description,
                    external_url: card.external_url,
                    image: card.image,
                    name: card.name,
                };
                Ok(json::Json(c))
            }
            Err(e) => Err(NotFound(e.to_string())),
        }
    }
}
