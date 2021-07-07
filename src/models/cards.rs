use crate::db::connect;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::cards;

#[derive(Queryable, Debug)]
pub struct CardEntity {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub image: String,
    pub sign: String,
    pub mnemonic: String,
    pub kana: String,
    pub deck_id: i32,
}

#[derive(Insertable)]
#[table_name = "cards"]
pub struct Card {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub image: String,
    pub sign: String,
    pub mnemonic: String,
    pub kana: String,
    pub deck_id: i32,
}

// Get NFT Meta Data by <token_id>
impl CardEntity {
    pub fn get_by_pk(token_id: u8) -> Result<CardEntity, diesel::result::Error> {
        use crate::schema::cards::dsl::cards;
        let conn = connect();
        cards.find(token_id as i32).first(&conn)
    }
}

//     let new_card = Card {
//         id: token_id as i32,
//         name: String::from("Name"),
//         description: String::from("Description"),
//         external_url: String::from("https://<EXTERNAL URL>"),
//         image: String::from("https://<IMAGE>"),
//         sign: String::from("Sign"),
//         mnemonic: String::from("Mnemonic"),
//         kana: String::from("Kana"),
//         deck_id: 0
//     };
//
//     use crate::schema::cards;
//     diesel::insert_into(cards::table)
//         .values(&new_card)
//         .get_result(&conn)
//         .expect("Error saving new card")
