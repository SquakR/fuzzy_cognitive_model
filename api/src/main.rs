#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use fuzzy_cognitive_model::cookies;
use fuzzy_cognitive_model::db;
use fuzzy_cognitive_model::errors::AppError;
use fuzzy_cognitive_model::models::User;
use fuzzy_cognitive_model::request_guards::UserAgent;
use fuzzy_cognitive_model::services::email_confirmation_services;
use fuzzy_cognitive_model::services::password_services;
use fuzzy_cognitive_model::services::session_services;
use fuzzy_cognitive_model::services::users_services;
use fuzzy_cognitive_model::storage::Storage;
use fuzzy_cognitive_model::types::{
    ChangeLanguageType, ChangePasswordType, CredentialsType, ResetPasswordType, SessionType,
    UserInChangeType, UserInCreateType, UserOutType,
};
use fuzzy_cognitive_model::utils;
use fuzzy_cognitive_model::utils::Operation;
use ipnetwork::IpNetwork;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;
use rocket_cors::AllowedOrigins;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_routes, openapi_spec};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

/// Create new user
#[openapi(tag = "users")]
#[post("/user", data = "<user_in>")]
async fn create_user(
    user_in: Form<UserInCreateType<'_>>,
    storage: &State<Storage>,
) -> Result<Json<UserOutType>, AppError> {
    let connection = &mut db::establish_connection();
    let user = users_services::create_user(connection, storage, user_in.into_inner()).await?;
    Ok(Json(UserOutType::from(user)))
}

/// Confirm user email
#[openapi(tag = "users")]
#[patch("/confirm_email/<token>")]
fn confirm_email(token: &str) -> Result<Json<UserOutType>, AppError> {
    let connection = &mut db::establish_connection();
    let user = email_confirmation_services::confirm_email(connection, token)?;
    Ok(Json(UserOutType::from(user)))
}

/// Get current user
#[openapi(tag = "users")]
#[get("/me")]
fn get_me(user: User) -> Json<UserOutType> {
    Json(UserOutType::from(user))
}

/// Change current user
#[openapi(tag = "users")]
#[put("/me", data = "<user_in>")]
async fn change_me(
    user_in: Form<UserInChangeType<'_>>,
    storage: &State<Storage>,
    user: User,
) -> Result<Json<UserOutType>, AppError> {
    let connection = &mut db::establish_connection();
    let user = users_services::change_user(connection, storage, user, user_in.into_inner()).await?;
    Ok(Json(UserOutType::from(user)))
}

/// Change current user language
#[openapi(tag = "users")]
#[patch("/me_language", format = "json", data = "<change_language>")]
fn change_me_language(
    change_language: Json<ChangeLanguageType>,
    user: User,
) -> Result<Json<UserOutType>, AppError> {
    let connection = &mut db::establish_connection();
    let user = users_services::change_user_language(
        connection,
        user,
        change_language.language.as_deref(),
    )?;
    Ok(Json(UserOutType::from(user)))
}

/// Change current user password
#[openapi(tag = "users")]
#[patch("/me_password", format = "json", data = "<change_password>")]
fn change_me_password(
    change_password: Json<ChangePasswordType>,
    user: User,
    cookies_jar: &CookieJar<'_>,
) -> Result<(), AppError> {
    let session_id = match cookies::get_session_id(cookies_jar) {
        Some(session_id) => session_id,
        None => return Err(AppError::BadRequestError),
    };
    let connection = &mut db::establish_connection();
    password_services::change_user_password(connection, &user, change_password.into_inner())?;
    users_services::sign_out(connection, &user, session_id)?;
    cookies::remove_session_id(cookies_jar);
    Ok(())
}

/// Request user password reset
#[openapi(tag = "users")]
#[post("/request_password_reset/<email>")]
async fn request_password_reset(email: &str, cookies_jar: &CookieJar<'_>) -> Result<(), AppError> {
    if cookies::get_session_id(cookies_jar).is_some() {
        return Err(AppError::ValidationError(String::from(
            "Unable to reset password with active session.",
        )));
    }
    let connection = &mut db::establish_connection();
    password_services::request_password_reset(connection, email).await?;
    Ok(())
}

/// Reset user password
#[openapi(tag = "users")]
#[patch("/reset_password", format = "json", data = "<reset_password>")]
fn reset_password(reset_password: Json<ResetPasswordType>) -> Result<(), AppError> {
    let connection = &mut db::establish_connection();
    password_services::reset_password(connection, reset_password.into_inner())
}

/// Create new session
#[openapi(tag = "users")]
#[post("/sign_in", format = "json", data = "<credentials>")]
fn sign_in(
    credentials: Json<CredentialsType>,
    cookies_jar: &CookieJar<'_>,
    ip_address: SocketAddr,
    user_agent: UserAgent,
) -> Result<Json<SessionType>, AppError> {
    if cookies::has_session_id(cookies_jar) {
        return Err(AppError::BadRequestError);
    }
    let connection = &mut db::establish_connection();
    let ip_address = match ip_address {
        SocketAddr::V4(v4) => IpNetwork::from(IpAddr::V4(v4.ip().clone())),
        SocketAddr::V6(v6) => IpNetwork::from(IpAddr::V6(v6.ip().clone())),
    };
    let session = users_services::sign_in(
        connection,
        credentials.into_inner(),
        &ip_address,
        &user_agent.0,
    )?;
    cookies::add_session_id(cookies_jar, session.id);
    let session_type = session_services::session_to_session_type(&session, session.id);
    Ok(Json(session_type))
}

/// Deactivate session
#[openapi(tag = "users")]
#[patch("/sign_out_multiple", format = "json", data = "<session_ids>")]
fn sign_out_multiple(session_ids: Json<Vec<i32>>, user: User) -> Result<(), AppError> {
    let connection = &mut db::establish_connection();
    for session_id in session_ids.into_inner() {
        users_services::sign_out(connection, &user, session_id)?;
    }
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
fn get_sessions(
    user: User,
    cookies_jar: &CookieJar<'_>,
) -> Result<Json<Vec<SessionType>>, AppError> {
    let session_id = match cookies::get_session_id(cookies_jar) {
        Some(session_id) => session_id,
        None => return Err(AppError::BadRequestError),
    };
    let connection = &mut db::establish_connection();
    let sessions = session_services::get_user_active_sessions(connection, user.id)?;
    let session_types = sessions
        .into_iter()
        .map(|session| session_services::session_to_session_type(&session, session_id))
        .collect::<Vec<SessionType>>();
    Ok(Json(session_types))
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

macro_rules! get_routes {
    ($first_route:expr, $($route:expr),*) => {{
        let settings = OpenApiSettings::new();
        let mut spec = openapi_spec![$first_route $(,$route)*](&settings);
        spec.info.title = String::from("Fuzzy Cognitive Model");
        utils::patch_wrong_content_type(&mut spec, "/user", Operation::Post);
        utils::patch_wrong_content_type(&mut spec, "/me", Operation::Put);
        openapi_routes![$first_route $(,$route)*](Some(spec), &settings)
    }};
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
        .mount(
            "/api/v1",
            get_routes!(
                create_user,
                confirm_email,
                get_me,
                change_me,
                change_me_language,
                change_me_password,
                request_password_reset,
                reset_password,
                sign_in,
                sign_out_multiple,
                sign_out,
                get_sessions,
                get_user_avatar
            ),
        )
        .mount("/api/v1/docs", make_swagger_ui(&get_docs()))
        .attach(cors)
}
