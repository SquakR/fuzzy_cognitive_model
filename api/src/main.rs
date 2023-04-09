#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use fuzzy_cognitive_model::locale::LocaleFairing;
use fuzzy_cognitive_model::plugins::{ControlVerticesPlugin, PluginsFairing};
use fuzzy_cognitive_model::response;
use fuzzy_cognitive_model::routes::MountRoutes;
use fuzzy_cognitive_model::storage::Storage;
use fuzzy_cognitive_model::utils;
use fuzzy_cognitive_model::web_socket::WebSocketListener;
use rocket::catcher::Catcher;
use rocket_cors::AllowedOrigins;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

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
        &serde_json::from_str::<Box<[String]>>(&utils::get_env("CORS_ALLOWED_ORIGINS")).unwrap(),
    );
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let web_socket_listener = WebSocketListener::new(
        String::from("127.0.0.1"),
        utils::get_env("WS_PORT").parse().expect("Invalid WS_PORT"),
    );

    let storage = Storage::new();

    rocket::build()
        .manage(storage)
        .mount_routes("/api/v1")
        .register(
            "/api/v1",
            vec![
                Catcher::new(400, response::handle_bad_request_error),
                Catcher::new(401, response::handle_unauthorized_error),
                Catcher::new(500, response::handle_internal_server_error),
            ],
        )
        .mount("/api/v1/docs", make_swagger_ui(&get_docs()))
        .attach(cors)
        .attach(LocaleFairing)
        .attach(PluginsFairing)
        .attach(ControlVerticesPlugin)
        .attach(web_socket_listener)
}
