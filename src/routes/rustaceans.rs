use crate::auth::BasicAuth;
use crate::models::rustacean::{NewRustacean, Rustacean};
use crate::repositories::rustacean::RustaceanRepository;
use rocket::serde::json::{json, Json, Value};

use rocket::http::Status;
use rocket::response::status::{self, Custom};

#[database("sqlite")] //drops from Rocket.toml file
pub struct DataBaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
pub async fn get_rustaceans(
    _auth: BasicAuth,
    db: DataBaseConnection,
) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        RustaceanRepository::find_multiple(connection, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DataBaseConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::find(connection, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    _auth: BasicAuth,
    db: DataBaseConnection,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        RustaceanRepository::create(connection, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    rustacean: Json<Rustacean>,
    db: DataBaseConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::update(connection, id, rustacean.into_inner())
            .map(|rastacean| json!(rastacean))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DataBaseConnection,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::delete(connection, id)
            .map(|_| status::NoContent)
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}
