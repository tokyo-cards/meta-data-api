#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod handlers;
mod models;
mod schema;
mod serializers;

use handlers::cards;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cards::get])
}
