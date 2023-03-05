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

use auth::BasicAuth;
use diesel::prelude::*;
use models::rustacean::{NewRustacean, Rustacean};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use schema::rustaceans;

#[database("sqlite")] //drops from Rocket.toml file
struct DataBaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustacens(_auth: BasicAuth, db: DataBaseConnection) -> Value {
    db.run(|connection| {
        let result = rustaceans::table
            .limit(100)
            .load::<Rustacean>(connection)
            .expect("Failed to read Rustaceans entries");
        json!(result)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacen(id: i32, _auth: BasicAuth, db: DataBaseConnection) -> Value {
    db.run(move |connection| {
        let result = rustaceans::table
            .find(&id)
            .get_result::<Rustacean>(connection)
            .expect("Faild retrieving rustacean row");
        json!(result)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacen(
    _auth: BasicAuth,
    db: DataBaseConnection,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Failed to inserting new rustaceans");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacen(
    id: i32,
    _auth: BasicAuth,
    rustacean: Json<Rustacean>,
    db: DataBaseConnection,
) -> Value {
    db.run(move |connection| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name.to_owned()),
            )) //tuple syntax
            .execute(connection)
            .expect("Faild updating rustacean entry");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacen(id: i32, _auth: BasicAuth, db: DataBaseConnection) -> status::NoContent {
    db.run(move |connection| {
        diesel::delete(rustaceans::table.find(id))
            .execute(connection)
            .expect("Faild delete rustaceans entry");
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
                get_rustacens,
                view_rustacen,
                create_rustacen,
                update_rustacen,
                delete_rustacen,
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
