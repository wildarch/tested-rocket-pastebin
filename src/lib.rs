#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::Rocket;

extern crate rocket_contrib;
extern crate uuid;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate r2d2;
extern crate r2d2_diesel;

pub mod schema;
pub mod models;

extern crate dotenv;

use std::env;
use std::path::PathBuf;

mod routes;
pub use routes::*;

pub mod database;


pub fn paste_dir() -> PathBuf {
    let mut path = env::current_dir()
        .expect("valid working directory");
    path.push("pastes");
    path
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index, upload_paste, get_paste])
        .manage(database::pool())
}
