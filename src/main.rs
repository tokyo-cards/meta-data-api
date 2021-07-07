#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

mod models;
mod serializers;
mod schema;
mod db;

use rocket::serde::{json};
use serializers::cards::CardSerialized;

#[get("/api/cards/<token_id>")]
fn cards(token_id: u8) -> json::Json<CardSerialized> {
    let card: CardSerialized = CardSerialized::get_by_pk(token_id);
    json::Json(card)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cards])
}
