#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use fuzzy_cognitive_model::cookies;
use fuzzy_cognitive_model::db;
use fuzzy_cognitive_model::errors::AppError;
use fuzzy_cognitive_model::models::Session;
use fuzzy_cognitive_model::models::User;
use fuzzy_cognitive_model::services::session_services;
use fuzzy_cognitive_model::services::users_services;
use fuzzy_cognitive_model::storage::Storage;
use fuzzy_cognitive_model::types::{Credentials, UserInCreate, UserInUpdate};
use fuzzy_cognitive_model::utils;
use fuzzy_cognitive_model::utils::Operation;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;
use rocket_cors::AllowedOrigins;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_routes, openapi_spec};
use std::path::PathBuf;

/// Create new user
#[openapi(tag = "users")]
#[post("/user", data = "<user_in>")]
async fn create_user(
    user_in: Form<UserInCreate<'_>>,
    storage: &State<Storage>,
) -> Result<Json<User>, AppError> {
    let connection = &mut db::establish_connection();
    let user = users_services::create_user(connection, storage, user_in.into_inner()).await?;
    Ok(Json(user))
}

/// Get current user
#[openapi(tag = "users")]
#[get("/me")]
fn get_me(user: User) -> Json<User> {
    Json(user)
}

/// Update an current user
#[openapi(tag = "users")]
#[put("/me", data = "<user_in>")]
async fn update_me(
    user_in: Form<UserInUpdate<'_>>,
    storage: &State<Storage>,
    user: User,
) -> Result<Json<User>, AppError> {
    let connection = &mut db::establish_connection();
    let user = users_services::update_user(connection, storage, user, user_in.into_inner()).await?;
    Ok(Json(user))
}

/// Create new session
#[openapi(tag = "users")]
#[post("/sign_in", format = "json", data = "<credentials>")]
fn sign_in(
    credentials: Json<Credentials>,
    cookies_jar: &CookieJar<'_>,
) -> Result<Json<Session>, AppError> {
    if cookies::has_session_id(cookies_jar) {
        return Err(AppError::BadRequestError);
    }
    let connection = &mut db::establish_connection();
    let session = users_services::sign_in(connection, credentials.into_inner())?;
    cookies::add_session_id(cookies_jar, session.id);
    Ok(Json(session))
}

/// Deactivate session
#[openapi(tag = "users")]
#[patch("/sign_out/<session_id>")]
fn sign_out_session(session_id: i32, user: User) -> Result<(), AppError> {
    let connection = &mut db::establish_connection();
    users_services::sign_out(connection, &user, session_id)?;
    Ok(())
}

/// Deactivate current session
#[openapi(tag = "users")]
#[patch("/sign_out")]
fn sign_out(user: User, cookies_jar: &CookieJar<'_>) -> Result<(), AppError> {
    let session_id = match cookies::get_session_id(cookies_jar) {
        Some(session_id) => session_id,
        None => return Err(AppError::BadRequestError),
    };
    let connection = &mut db::establish_connection();
    users_services::sign_out(connection, &user, session_id)?;
    cookies::remove_session_id(cookies_jar);
    Ok(())
}

/// Get user sessions
#[openapi(tag = "users")]
#[get("/sessions")]
fn get_sessions(user: User) -> Result<Json<Vec<Session>>, AppError> {
    let connection = &mut db::establish_connection();
    let sessions = session_services::get_user_active_sessions(connection, user.id)?;
    Ok(Json(sessions))
}

/// Get user avatar
#[openapi(tag = "users")]
#[get("/storage/user_avatars/<path..>")]
async fn get_user_avatar(
    path: PathBuf,
    _user: User,
    storage: &State<Storage>,
) -> Result<NamedFile, AppError> {
    storage.get_user_avatar(path).await
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: String::from("/api/v1/openapi.json"),
        ..Default::default()
    }
}

fn get_routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings::new();
    let mut spec = openapi_spec![
        create_user,
        get_me,
        update_me,
        sign_in,
        sign_out_session,
        sign_out,
        get_sessions,
        get_user_avatar
    ](&settings);
    spec.info.title = String::from("Fuzzy Cognitive Model");
    utils::patch_wrong_content_type(&mut spec, "/user", Operation::Post);
    utils::patch_wrong_content_type(&mut spec, "/me", Operation::Put);
    let routes = openapi_routes![
        create_user,
        get_me,
        update_me,
        sign_in,
        sign_out_session,
        sign_out,
        get_sessions,
        get_user_avatar
    ](Some(spec), &settings);
    return routes;
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

    let figment = rocket::Config::figment().merge(("secret_key", utils::get_env("SECRET_KEY")));

    let storage = Storage::new();

    rocket::custom(figment)
        .manage(storage)
        .mount("/api/v1", get_routes())
        .mount("/api/v1/docs", make_swagger_ui(&get_docs()))
        .attach(cors)
}
