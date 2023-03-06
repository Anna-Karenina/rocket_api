#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::fairing::AdHoc;
use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};

mod auth;
mod schema;
mod models {
    pub mod rustacean;
}
mod repositories {
    pub mod rustacean;
}

mod routes {
    pub mod rustaceans;
}

embed_migrations!();

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

async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    routes::rustaceans::DataBaseConnection::get_one(&rocket)
        .await
        .expect("failed to retrive database connection")
        .run(|connect| match embedded_migrations::run(connect) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                routes::rustaceans::get_rustaceans,
                routes::rustaceans::view_rustacean,
                routes::rustaceans::create_rustacean,
                routes::rustaceans::update_rustacean,
                routes::rustaceans::delete_rustacean,
            ],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_content],
        )
        .attach(routes::rustaceans::DataBaseConnection::fairing())
        .attach(AdHoc::try_on_ignite(
            "Running DB migrations",
            run_db_migrations,
        ))
        .launch()
        .await;
}
