#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::{Rocket, Data};

extern crate uuid;
use uuid::Uuid;

use std::env;
use std::path::PathBuf;
use std::io;
use std::fs::{self, File};

pub const HOME_TEXT : &'static str = "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
";

pub fn get_paste_dir() -> PathBuf {
    let mut path = env::current_dir().expect("valid working directory");
    path.push("pastes");
    path
}

#[get("/")]
fn index() -> &'static str {
    HOME_TEXT
}

#[get("/<id>")]
fn get_paste(id: String) -> Option<File> {
   let mut paste_path = get_paste_dir();
   paste_path.push(id);
   File::open(paste_path).ok()
}

#[post("/", data = "<paste>")]
fn upload_paste(paste: Data) -> io::Result<String> {
    let id = Uuid::new_v4().hyphenated().to_string();
    let mut paste_path = get_paste_dir();
    fs::create_dir_all(&paste_path)?;
    paste_path.push(&id);
    paste.stream_to_file(paste_path)?;
    Ok(id)
}

pub fn get_rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index, upload_paste, get_paste])
}
