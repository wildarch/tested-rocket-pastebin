use uuid::Uuid;
use rocket_contrib::UUID;

use ::*;

use database::DbConn;

use ::schema::pastes::dsl::*;
use diesel::prelude::*;
use ::models::Paste;
use diesel::associations::HasTable;

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

#[get("/<paste_id>")]
fn get_paste(paste_id: UUID, db: DbConn) -> Option<String> {
    pastes.filter(id.eq(paste_id.into_inner()))
        .limit(1).load(&*db).ok()
        .and_then(|rs: Vec<Paste> | rs.into_iter().next())
        .and_then(|r| Some(r.body.clone()))
}

#[post("/", data = "<paste_body>")]
fn upload_paste(paste_body: String, db: DbConn) -> Result<String, diesel::result::Error> {
    let paste_id = Uuid::new_v4();
    diesel::insert(&Paste {
        id: paste_id,
        body: paste_body
    }).into(pastes::table()).execute(&*db)
    .map(|_| paste_id.hyphenated().to_string())
}
