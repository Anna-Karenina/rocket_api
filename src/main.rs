#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};

#[get("/rustaceans")]
fn get_rustacens() -> Value {
    json!([
    {"id": 1, "name": "John Doe"},
    {"id": 2, "name": "John Doe next"},
    ])
}

#[get("/rustaceans/<id>")]
fn view_rustacen(id: i32) -> Value {
    json!({
      "id": id,
      "name": "John Doe",
      "email":  "John_Doe@duck.com"
    })
}

#[post("/rustaceans", format = "json")]
fn create_rustacen() -> Value {
    json!({"id": 3, "name": "John Doe third", "email":  "John_Doe@duck.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacen(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email":  "John_Doe@duck.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacen(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
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
        .register("/", catchers![not_found])
        .launch()
        .await;
}
