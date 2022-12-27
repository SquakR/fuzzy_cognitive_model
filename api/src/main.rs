#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use fuzzy_cognitive_model::db;
use fuzzy_cognitive_model::errors::AppError;
use fuzzy_cognitive_model::models::User;
use fuzzy_cognitive_model::services;
use rocket::serde::json::Json;
use rocket_cors::AllowedOrigins;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use std::env;

/// Get an user by id
#[openapi(tag = "users")]
#[get("/user/<user_id>")]
fn user(user_id: i32) -> Result<Json<User>, AppError> {
    let connection = &mut db::establish_connection();
    let user = services::get_user(connection, user_id)?;
    Ok(Json(user))
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: String::from("/api/v1/openapi.json"),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().unwrap();

    let allowed_origins = AllowedOrigins::some_exact(
        &serde_json::from_str::<Box<[String]>>(&env::var("CORS_ALLOWED_ORIGINS").unwrap()).unwrap(),
    );
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .mount("/api/v1", openapi_get_routes![user])
        .mount("/api/v1/docs", make_swagger_ui(&get_docs()))
        .attach(cors)
}
