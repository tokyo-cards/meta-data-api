use crate::serializers::cards::{CardResultJSON, CardSerialized};

#[get("/api/cards/<token_id>")]
pub fn get(token_id: u8) -> CardResultJSON {
    CardSerialized::get_by_pk(token_id)
}
