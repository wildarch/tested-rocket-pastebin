use uuid::Uuid;
use std::fs::{self, File};

use ::*;
use std::io;

use rocket::Data;

use database::DbConn;

pub const HOME_TEXT : &'static str = "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
";

#[get("/")]
fn index() -> &'static str {
    HOME_TEXT
}

#[get("/<id>")]
fn get_paste(id: String, db: DbConn) -> Option<File> {
   let mut paste_path = paste_dir();
   paste_path.push(id);
   File::open(paste_path).ok()
}

#[post("/", data = "<paste>")]
fn upload_paste(paste: Data, db: DbConn) -> io::Result<String> {
    let id = Uuid::new_v4().hyphenated().to_string();
    let mut paste_path = paste_dir();
    fs::create_dir_all(&paste_path)?;
    paste_path.push(&id);
    paste.stream_to_file(paste_path)?;
    Ok(id)
}
