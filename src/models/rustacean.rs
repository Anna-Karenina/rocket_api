use crate::schema::rustaceans;
#[derive(Queryable, serde::Deserialize, serde::Serialize)]

pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}
#[derive(serde::Deserialize, Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    name: String,
    email: String,
}
