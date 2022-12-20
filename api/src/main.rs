#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use fuzzy_cognitive_model::db;
use fuzzy_cognitive_model::errors::QueryError;
use fuzzy_cognitive_model::models::User;
use fuzzy_cognitive_model::services;
use rocket::serde::json::Json;

#[get("/user/<user_id>")]
fn user(user_id: i32) -> Result<Json<User>, QueryError> {
    let connection = &mut db::establish_connection();
    let user = services::get_user(connection, user_id)?;
    Ok(Json(user))
}

#[launch]
fn rocket() -> _ {
    dotenv().unwrap();
    rocket::build().mount("/api/v1", routes![user])
}
