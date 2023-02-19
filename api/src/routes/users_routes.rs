use crate::cookies;
use crate::db;
use crate::models::User;
use crate::request_guards::{AcceptLanguage, Locale, UserAgent, UserLocale};
use crate::response::{AppError, PathResult};
use crate::services::email_confirmation_services;
use crate::services::password_services;
use crate::services::session_services;
use crate::services::users_services;
use crate::storage::Storage;
use crate::types::{
    ChangeLanguageType, ChangePasswordType, CredentialsType, ResetPasswordType, SessionType,
    UserInChangeType, UserInCreateType, UserOutType,
};
use ipnetwork::IpNetwork;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use rust_i18n::t;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

/// Create new user
#[openapi(tag = "users")]
#[post("/user", data = "<user_in>")]
pub async fn create_user(
    user_in: Form<UserInCreateType<'_>>,
    cookies_jar: &CookieJar<'_>,
    storage: &State<Storage>,
    locale: Locale,
) -> PathResult<Json<UserOutType>, Locale> {
    if cookies::get_session_id(cookies_jar).is_some() {
        return PathResult::new(
            Err(AppError::ValidationError(Box::new(|locale| {
                t!("create_user_active_session_error", locale = locale)
            }))),
            locale,
        );
    }
    let connection = &mut db::establish_connection();
    let user = match users_services::create_user(connection, storage, user_in.into_inner()).await {
        Ok(user) => user,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(UserOutType::from(user))), locale)
}

/// Confirm user email
#[openapi(tag = "users")]
#[patch("/confirm_email/<token>")]
pub fn confirm_email(
    token: &str,
    accept_language: &AcceptLanguage,
    locale: Locale,
) -> PathResult<Json<UserOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let user = match email_confirmation_services::confirm_email(connection, token) {
        Ok(user) => user,
        Err(app_error) => return PathResult::new(Err(app_error), UserLocale(locale.0.to_owned())),
    };
    let locale = UserLocale::new(&user, accept_language);
    PathResult::new(Ok(Json(UserOutType::from(user))), locale)
}

/// Get current user
#[openapi(tag = "users")]
#[get("/me")]
pub fn get_me(user: User, locale: UserLocale) -> PathResult<Json<UserOutType>, UserLocale> {
    PathResult::new(Ok(Json(UserOutType::from(user))), locale)
}

/// Change current user
#[openapi(tag = "users")]
#[put("/me", data = "<user_in>")]
pub async fn change_me(
    user_in: Form<UserInChangeType<'_>>,
    storage: &State<Storage>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<UserOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let user =
        match users_services::change_user(connection, storage, user, user_in.into_inner()).await {
            Ok(user) => user,
            Err(app_error) => return PathResult::new(Err(app_error), locale),
        };
    PathResult::new(Ok(Json(UserOutType::from(user))), locale)
}

/// Change current user language
#[openapi(tag = "users")]
#[patch("/me_language", format = "json", data = "<change_language>")]
pub fn change_me_language(
    change_language: Json<ChangeLanguageType>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<UserOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let user = match users_services::change_user_language(
        connection,
        user,
        change_language.language.as_deref(),
    ) {
        Ok(user) => user,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(UserOutType::from(user))), locale)
}

/// Change current user password
#[openapi(tag = "users")]
#[patch("/me_password", format = "json", data = "<change_password>")]
pub fn change_me_password(
    change_password: Json<ChangePasswordType>,
    cookies_jar: &CookieJar<'_>,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let session_id = match cookies::get_session_id(cookies_jar) {
        Some(session_id) => session_id,
        None => return PathResult::new(Err(AppError::InternalServerError), locale),
    };
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        password_services::change_user_password(connection, &user, change_password.into_inner())
    {
        return PathResult::new(Err(app_error), locale);
    }
    if let Err(app_error) = users_services::sign_out(connection, &user, session_id) {
        return PathResult::new(Err(app_error), locale);
    }
    cookies::remove_session_id(cookies_jar);
    PathResult::new(Ok(()), locale)
}

/// Request user password reset
#[openapi(tag = "users")]
#[post("/request_password_reset/<email>")]
pub async fn request_password_reset(
    email: &str,
    cookies_jar: &CookieJar<'_>,
    locale: Locale,
) -> PathResult<(), Locale> {
    if cookies::get_session_id(cookies_jar).is_some() {
        return PathResult::new(
            Err(AppError::ValidationError(Box::new(|locale| {
                t!("reset_password_active_session_error", locale = locale)
            }))),
            locale,
        );
    }
    let connection = &mut db::establish_connection();
    if let Err(app_error) = password_services::request_password_reset(connection, email).await {
        return PathResult::new(Err(app_error), locale);
    };
    PathResult::new(Ok(()), locale)
}

/// Reset user password
#[openapi(tag = "users")]
#[patch("/reset_password", format = "json", data = "<reset_password>")]
pub fn reset_password(
    reset_password: Json<ResetPasswordType>,
    locale: Locale,
) -> PathResult<(), Locale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        password_services::reset_password(connection, reset_password.into_inner())
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Create new session
#[openapi(tag = "users")]
#[post("/sign_in", format = "json", data = "<credentials>")]
pub fn sign_in(
    credentials: Json<CredentialsType>,
    cookies_jar: &CookieJar<'_>,
    ip_address: SocketAddr,
    user_agent: UserAgent,
    locale: Locale,
) -> PathResult<Json<SessionType>, Locale> {
    if cookies::has_session_id(cookies_jar) {
        return PathResult::new(
            Err(AppError::ValidationError(Box::new(|locale| {
                t!("sign_in_active_session_error", locale = locale)
            }))),
            locale,
        );
    }
    let connection = &mut db::establish_connection();
    let ip_address = match ip_address {
        SocketAddr::V4(v4) => IpNetwork::from(IpAddr::V4(v4.ip().clone())),
        SocketAddr::V6(v6) => IpNetwork::from(IpAddr::V6(v6.ip().clone())),
    };
    let session = match users_services::sign_in(
        connection,
        credentials.into_inner(),
        &ip_address,
        &user_agent.0,
    ) {
        Ok(session) => session,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    cookies::add_session_id(cookies_jar, session.id);
    let session_type = session_services::session_to_session_type(&session, session.id);
    PathResult::new(Ok(Json(session_type)), locale)
}

/// Deactivate session
#[openapi(tag = "users")]
#[patch("/sign_out_multiple", format = "json", data = "<session_ids>")]
pub fn sign_out_multiple(
    session_ids: Json<Vec<i32>>,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    for session_id in session_ids.into_inner() {
        if let Err(app_error) = users_services::sign_out(connection, &user, session_id) {
            return PathResult::new(Err(app_error), locale);
        }
    }
    PathResult::new(Ok(()), locale)
}

/// Deactivate current session
#[openapi(tag = "users")]
#[patch("/sign_out")]
pub fn sign_out(
    cookies_jar: &CookieJar<'_>,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let session_id = match cookies::get_session_id(cookies_jar) {
        Some(session_id) => session_id,
        None => return PathResult::new(Err(AppError::InternalServerError), locale),
    };
    let connection = &mut db::establish_connection();
    if let Err(app_error) = users_services::sign_out(connection, &user, session_id) {
        return PathResult::new(Err(app_error), locale);
    }
    cookies::remove_session_id(cookies_jar);
    PathResult::new(Ok(()), locale)
}

/// Get user sessions
#[openapi(tag = "users")]
#[get("/sessions")]
pub fn get_sessions(
    cookies_jar: &CookieJar<'_>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<Vec<SessionType>>, UserLocale> {
    let session_id = match cookies::get_session_id(cookies_jar) {
        Some(session_id) => session_id,
        None => return PathResult::new(Err(AppError::InternalServerError), locale),
    };
    let connection = &mut db::establish_connection();
    let sessions = match session_services::get_user_active_sessions(connection, user.id) {
        Ok(sessions) => sessions,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    let session_types = sessions
        .into_iter()
        .map(|session| session_services::session_to_session_type(&session, session_id))
        .collect::<Vec<SessionType>>();
    PathResult::new(Ok(Json(session_types)), locale)
}

/// Get user avatar
#[openapi(tag = "users")]
#[get("/storage/user_avatars/<path..>")]
pub async fn get_user_avatar(
    path: PathBuf,
    storage: &State<Storage>,
    locale: UserLocale,
) -> PathResult<NamedFile, UserLocale> {
    PathResult::new(storage.get_user_avatar(path).await, locale)
}
