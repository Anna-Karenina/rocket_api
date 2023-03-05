#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod schema;
mod models {
    pub mod rustacean;
}
mod repositories {
    pub mod rustacean;
}

use auth::BasicAuth;
use models::rustacean::{NewRustacean, Rustacean};
use repositories::rustacean::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::{json, Json, Value};

#[database("sqlite")] //drops from Rocket.toml file
struct DataBaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DataBaseConnection) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        RustaceanRepository::find_multiple(connection, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(
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
async fn create_rustacean(
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
async fn update_rustacean(
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
async fn delete_rustacean(
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

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}

#[catch(422)]
fn unprocessable_content() -> Value {
    json!("Invalid entity. Probably some missing fields?")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean,
            ],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_content],
        )
        .attach(DataBaseConnection::fairing())
        .launch()
        .await;
}
