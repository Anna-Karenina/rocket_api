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
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[database("sqlite")] //drops from Rocket.toml file
struct DataBaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DataBaseConnection) -> Value {
    db.run(|connection| {
        let result = RustaceanRepository::find_multiple(connection, 100)
            .expect("Failed to read Rustaceans entries");
        json!(result)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DataBaseConnection) -> Value {
    db.run(move |connection| {
        let result =
            RustaceanRepository::find(connection, id).expect("Faild retrieving rustacean row");
        json!(result)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DataBaseConnection,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|connection| {
        let result = RustaceanRepository::create(connection, new_rustacean.into_inner())
            .expect("Failed to inserting new rustaceans");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    rustacean: Json<Rustacean>,
    db: DataBaseConnection,
) -> Value {
    db.run(move |connection| {
        let result = RustaceanRepository::update(connection, id, rustacean.into_inner())
            .expect("Faild updating rustacean entry");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DataBaseConnection) -> status::NoContent {
    db.run(move |connection| {
        RustaceanRepository::delete(connection, id).expect("Faild delete rustaceans entry");
        status::NoContent
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
