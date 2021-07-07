#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod schema;
mod serializers;

use rocket::response::status::NotFound;
use rocket::serde::json;
use serializers::cards::CardSerialized;

#[get("/api/cards/<token_id>")]
fn cards(token_id: u8) -> Result<json::Json<CardSerialized>, NotFound<String>> {
    CardSerialized::get_by_pk(token_id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cards])
}
