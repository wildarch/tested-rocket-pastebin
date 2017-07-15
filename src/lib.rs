#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::Rocket;


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


pub fn get_rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index])
}
