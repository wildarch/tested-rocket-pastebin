use uuid::Uuid;
use super::schema::pastes;

#[derive(Insertable)]
#[table_name="pastes"]
#[derive(Queryable)]
pub struct Paste {
    pub id: Uuid,
    pub body: String,
}
