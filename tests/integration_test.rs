extern crate tested_rocket_pastebin;
use tested_rocket_pastebin::{rocket, paste_dir, HOME_TEXT, database};

extern crate rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

extern crate tempdir;
use tempdir::TempDir;

extern crate uuid;
use uuid::Uuid;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::env;
use std::mem::forget;

extern crate dotenv;
use dotenv::dotenv;

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
fn it_shows_homepage() {
   let client = get_client();
   let mut res = client.get("/").dispatch();
   assert_eq!(res.status(), Status::Ok);
   assert_eq!(res.content_type().expect("valid Content-Type"), ContentType::Plain);
   let body = res.body_string();
   assert_eq!(body.expect("body content"), HOME_TEXT);
}

#[test]
fn it_uploads_paste() {
    let body = "Hello, world!";
    let client = get_client();
    let mut res = client.post("/")
        .body(body)
        .header(ContentType::Plain)
        .dispatch();
    let id = res.body_string().expect("id in response body");
    let mut paste_path = paste_dir();
    
    paste_path.push(id);
    let mut paste_file = File::open(paste_path).expect("paste file exists");
    let mut saved_body = String::new();
    paste_file.read_to_string(&mut saved_body).expect("paste file readable");
    assert_eq!(body, saved_body);
}

#[test]
fn it_shows_paste() {
    let client = get_client();
    let body = "Hello, world!";
    let id = Uuid::new_v4().hyphenated().to_string();
    let mut paste_path = paste_dir();
    fs::create_dir_all(&paste_path).expect("paste path created");
    paste_path.push(&id);
    {
        let mut file = File::create(paste_path).expect("paste file newly created");
        file.write_all(body.as_bytes()).expect("write expected body to paste file");
    }

    let mut url = String::from("/");
    url.push_str(&id);
    let mut res = client.get(url).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string().expect("present body"), body);
}
