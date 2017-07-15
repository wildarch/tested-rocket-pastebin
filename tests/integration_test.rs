extern crate rocket_pastebin;
use rocket_pastebin::{get_rocket, HOME_TEXT};

extern crate rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

fn get_client() -> Client {
    let rocket = get_rocket();
    Client::new(rocket).expect("valid rocket instance")
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
