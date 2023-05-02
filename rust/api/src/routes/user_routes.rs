use crate::cookies;
use crate::db;
use crate::locale::Locale;
use crate::models::User;
use crate::request::{AcceptLanguage, UserAgent};
use crate::response::{
    AppError, PathAnyResult, PathEmptyResult, PathResult, ToPathEmptyResult, ToPathResult,
    ToServiceResult,
};
use crate::services::{
    email_confirmation_services, password_services, session_services, user_services,
};
use crate::storage::Storage;
use crate::types::{
    ChangeLanguageType, ChangePasswordType, CredentialsType, PaginationInType, PaginationOutType,
    ResetPasswordType, SessionType, UserInChangeType, UserInCreateType, UserOutType,
};
use crate::web_socket::WebSocketProjectService;
use crate::{get_session_id, internal_server_error, validation_error};
use ipnetwork::IpNetwork;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

/// Create new user
#[openapi(tag = "users")]
#[post("/user", data = "<user_in>")]
pub async fn create_user(
    user_in: Form<UserInCreateType<'_>>,
    cookies_jar: &CookieJar<'_>,
    storage: &State<Storage>,
) -> PathResult<UserOutType> {
    if get_session_id!(cookies_jar).is_some() {
        return validation_error!("create_user_active_session_error");
    }
    let conn = &mut db::establish_connection();
    let user = user_services::create_user(conn, storage, user_in.into_inner()).await?;
    Ok(Json(UserOutType::from(user)))
}

/// Get users
#[openapi(tag = "users")]
#[get("/users?<search>&<page>&<per_page>")]
pub fn get_users(
    search: Option<&str>,
    page: Option<u16>,
    per_page: Option<u16>,
    _user: User,
) -> PathResult<PaginationOutType<UserOutType>> {
    let conn = &mut db::establish_connection();
    let pagination_in = PaginationInType {
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(15),
    };
    user_services::paginate_users(conn, search.map(|s| s.to_owned()), pagination_in)
        .to_path_result()
}

/// Confirm user email
#[openapi(tag = "users")]
#[patch("/confirm_email/<token>")]
pub fn confirm_email(
    token: &str,
    accept_language: &AcceptLanguage,
    locale: &Locale,
) -> PathResult<UserOutType> {
    let conn = &mut db::establish_connection();
    let user = email_confirmation_services::confirm_email(conn, token)?;
    locale.set_from_user(&user, accept_language);
    Ok(Json(UserOutType::from(user)))
}

/// Get current user
#[openapi(tag = "users")]
#[get("/me")]
pub fn get_me(user: User) -> PathResult<UserOutType> {
    Ok(Json(UserOutType::from(user)))
}

/// Change current user
#[openapi(tag = "users")]
#[put("/me", data = "<user_in>")]
pub async fn change_me(
    user_in: Form<UserInChangeType<'_>>,
    storage: &State<Storage>,
    user: User,
) -> PathResult<UserOutType> {
    let conn = &mut db::establish_connection();
    let user = user_services::change_user(conn, storage, user, user_in.into_inner()).await?;
    Ok(Json(UserOutType::from(user)))
}

/// Change current user language
#[openapi(tag = "users")]
#[patch("/me/language", format = "json", data = "<change_language>")]
pub fn change_me_language(
    change_language: Json<ChangeLanguageType>,
    accept_language: &AcceptLanguage,
    user: User,
    locale: &Locale,
) -> PathResult<UserOutType> {
    let conn = &mut db::establish_connection();
    let user =
        user_services::change_user_language(conn, user, change_language.language.as_deref())?;
    locale.set_from_user(&user, accept_language);
    Ok(Json(UserOutType::from(user)))
}

/// Change current user password
#[openapi(tag = "users")]
#[patch("/me/password", format = "json", data = "<change_password>")]
pub async fn change_me_password(
    change_password: Json<ChangePasswordType>,
    cookies_jar: &CookieJar<'_>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathEmptyResult {
    let session_id = match get_session_id!(cookies_jar) {
        Some(session_id) => session_id,
        None => return internal_server_error!(),
    };
    let conn = &mut db::establish_connection();
    password_services::change_user_password(
        conn,
        project_service,
        &user,
        session_id,
        change_password.into_inner(),
    )
    .await?;
    cookies::remove_session_id(cookies_jar);
    Ok(())
}

/// Request user password reset
#[openapi(tag = "users")]
#[post("/request_password_reset/<email>")]
pub async fn request_password_reset(email: &str, cookies_jar: &CookieJar<'_>) -> PathEmptyResult {
    if get_session_id!(cookies_jar).is_some() {
        return validation_error!("reset_password_active_session_error");
    }
    let conn = &mut db::establish_connection();
    password_services::request_password_reset(conn, email)
        .await
        .to_path_empty_result()
}

/// Reset user password
#[openapi(tag = "users")]
#[patch("/reset_password", format = "json", data = "<reset_password>")]
pub fn reset_password(reset_password: Json<ResetPasswordType>) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    password_services::reset_password(conn, reset_password.into_inner()).to_path_empty_result()
}

/// Create new session
#[openapi(tag = "users")]
#[post("/sign_in", format = "json", data = "<credentials>")]
pub fn sign_in(
    credentials: Json<CredentialsType>,
    cookies_jar: &CookieJar<'_>,
    ip_address: SocketAddr,
    user_agent: UserAgent,
) -> PathResult<SessionType> {
    if cookies::has_session_id(cookies_jar) {
        return validation_error!("sign_in_active_session_error");
    }
    let conn = &mut db::establish_connection();
    let ip_address = match ip_address {
        SocketAddr::V4(v4) => IpNetwork::from(IpAddr::V4(v4.ip().clone())),
        SocketAddr::V6(v6) => IpNetwork::from(IpAddr::V6(v6.ip().clone())),
    };
    let session =
        session_services::sign_in(conn, credentials.into_inner(), &ip_address, &user_agent.0)?;
    cookies::add_session_id(cookies_jar, session.id);
    let session_type = session_services::session_to_session_type(&session, session.id);
    Ok(Json(session_type))
}

/// Deactivate multiple sessions
#[openapi(tag = "users")]
#[patch("/sign_out_multiple", format = "json", data = "<session_ids>")]
pub async fn sign_out_multiple(
    session_ids: Json<Vec<i32>>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    let session_ids = session_ids.into_inner();
    session_services::check_user_sessions(conn, &user, &session_ids)?;
    session_services::sign_out(conn, project_service, &session_ids)
        .await
        .to_service_result()
        .to_path_empty_result()
}

/// Deactivate current session
#[openapi(tag = "users")]
#[patch("/sign_out")]
pub async fn sign_out(
    cookies_jar: &CookieJar<'_>,
    _user: User,
    project_service: WebSocketProjectService,
) -> PathEmptyResult {
    let session_id = match get_session_id!(cookies_jar) {
        Some(session_id) => session_id,
        None => return internal_server_error!(),
    };
    let conn = &mut db::establish_connection();
    session_services::sign_out(conn, project_service, &[session_id])
        .await
        .to_service_result()?;
    cookies::remove_session_id(cookies_jar);
    Ok(())
}

/// Get user sessions
#[openapi(tag = "users")]
#[get("/sessions")]
pub fn get_sessions(cookies_jar: &CookieJar<'_>, user: User) -> PathResult<Vec<SessionType>> {
    let session_id = match get_session_id!(cookies_jar) {
        Some(session_id) => session_id,
        None => return internal_server_error!(),
    };
    let conn = &mut db::establish_connection();
    let sessions = session_services::get_user_active_sessions(conn, user.id).to_service_result()?;
    let session_types = sessions
        .into_iter()
        .map(|session| session_services::session_to_session_type(&session, session_id))
        .collect::<Vec<SessionType>>();
    Ok(Json(session_types))
}

/// Get user avatar
#[openapi(tag = "users")]
#[get("/storage/user_avatars/<path..>")]
pub async fn get_user_avatar(
    path: PathBuf,
    storage: &State<Storage>,
    _user: User,
) -> PathAnyResult<NamedFile> {
    Ok(storage.get_user_avatar(path).await?)
}
