use uuid::Uuid;

#[derive(Queryable)]
pub struct Paste {
    pub id: Uuid,
    pub body: String,
}
