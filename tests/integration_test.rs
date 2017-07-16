extern crate tested_rocket_pastebin;
use tested_rocket_pastebin::{rocket, HOME_TEXT, database};
use tested_rocket_pastebin::models::*;
use tested_rocket_pastebin::schema::pastes::dsl::*;

extern crate rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

extern crate tempdir;
use tempdir::TempDir;

extern crate uuid;
use uuid::Uuid;

use std::env;
use std::mem::forget;

extern crate dotenv;
use dotenv::dotenv;

extern crate diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;


fn get_client() -> Client {
    setup_environment();
    let rocket = rocket();
    Client::new(rocket).expect("valid rocket instance")
}

fn setup_environment() {
    setup_database();
    setup_working_dir();
}


fn setup_database() {
    dotenv().ok();
    match env::var("DEV_DATABASE_URL") {
        Ok(u) => {
            env::set_var("DATABASE_URL", u);
        }
        Err(_) => {}
    }
    // Drop all tables
}

fn setup_working_dir() {
    // Put all pastes in a temporary directory instead of cluttering the working directory
    let tmp_dir = TempDir::new("pastebin-cwd").expect("valid temporary working directory");
    env::set_current_dir(&tmp_dir).expect("correctly set working directory");
    // Don't drop tmp_dir, so we keep the directory around
    forget(tmp_dir)
}


#[test]
fn it_starts_clean() {
    let conn = database::connect();
    let results: Vec<Paste> = pastes.load(&conn).expect("Succesfull query");
    assert!(results.is_empty());
}

#[test]
fn it_shows_homepage() {
   let client = get_client();
   let mut res = client.get("/").dispatch();
   assert_eq!(res.status(), Status::Ok);
   assert_eq!(res.content_type().expect("valid Content-Type"), ContentType::Plain);
   let paste_body = res.body_string();
   assert_eq!(paste_body.expect("body content"), HOME_TEXT);
}

#[test]
fn it_uploads_paste() {
    let paste_body = "Hello, world!";
    let client = get_client();
    let mut res = client.post("/")
        .body(paste_body)
        .header(ContentType::Plain)
        .dispatch();
    let paste_id = res.body_string().expect("id in response body");
    let parsed_paste_id: Uuid = Uuid::parse_str(&paste_id).expect("valid uuid returned");

    let conn = database::connect();
    let results : Vec<Paste> = pastes.filter(id.eq(parsed_paste_id)).load(&conn)
        .expect("succesful database query");
    assert_eq!(results.len(), 1);
    let saved = &results[0];
    assert_eq!(paste_body, saved.body);
}

#[test]
fn it_shows_paste() {
    let client = get_client();
    let paste_body = String::from("Hello, world!");
    let paste_id = Uuid::new_v4();

    let conn = database::connect();
    diesel::insert(&Paste {
        id: paste_id,
        body: paste_body.clone()
    }).into(pastes::table()).execute(&conn)
        .expect("Succesfully inserted paste");

    let mut url = String::from("/");
    url.push_str(&paste_id.hyphenated().to_string());
    let mut res = client.get(url).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string().expect("present body"), paste_body);
}
