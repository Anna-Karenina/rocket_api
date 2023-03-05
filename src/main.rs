#[macro_use]
extern crate rocket;
extern crate diesel;

#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::response::status;
use rocket::serde::json::{json, Value};

mod auth;
use auth::BasicAuth;

#[database("sqlite")] //drops from Rocket.toml file
struct DataBaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
fn get_rustacens(_auth: BasicAuth, _db: DataBaseConnection) -> Value {
    json!([
    {"id": 1, "name": "John Doe"},
    {"id": 2, "name": "John Doe next"},
    ])
}

#[get("/rustaceans/<id>")]
fn view_rustacen(id: i32, _auth: BasicAuth) -> Value {
    json!({
      "id": id,
      "name": "John Doe",
      "email":  "John_Doe@duck.com"
    })
}

#[post("/rustaceans", format = "json")]
fn create_rustacen(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe third", "email":  "John_Doe@duck.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacen(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email":  "John_Doe@duck.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacen(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustacens,
                view_rustacen,
                create_rustacen,
                update_rustacen,
                delete_rustacen,
            ],
        )
        .register("/", catchers![not_found, unauthorized])
        .attach(DataBaseConnection::fairing())
        .launch()
        .await;
}
