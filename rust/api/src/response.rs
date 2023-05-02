use crate::locale::Locale;
use crate::request::AcceptLanguage;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use okapi::openapi3::Responses;
use rocket::catcher::BoxFuture;
use rocket::http::hyper::header;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket_accept_language::AcceptLanguage as RocketAcceptLanguage;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::{response::OpenApiResponderInner, Result as RocketOkapiResult};

#[macro_export]
macro_rules! validation_error {
    ($key:expr) => {
        Err(crate::response::AppError::ValidationError(Box::new(move |locale| {
            t!($key, locale = locale)
        })))
    };
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        Err(crate::response::AppError::ValidationError(Box::new(move |locale| {
            t!($key, locale = locale, $($var_name = $var_val),*)
        })))
    };
}

#[macro_export]
macro_rules! forbidden_error {
    ($key:expr) => {
        Err(crate::response::AppError::ForbiddenError(String::from(
            $key,
        )))
    };
}

#[macro_export]
macro_rules! not_found_error {
    ($key:expr) => {
        Err(crate::response::AppError::NotFoundError(String::from($key)))
    };
}

#[macro_export]
macro_rules! internal_server_error {
    () => {
        Err(AppError::InternalServerError)
    };
}

pub type ServiceResult<T> = Result<T, AppError>;

pub type PathResult<T> = Result<Json<T>, AppError>;
pub type PathAnyResult<T> = Result<T, AppError>;
pub type PathEmptyResult = Result<(), AppError>;

pub trait ToPathResult<T> {
    fn to_path_result(self) -> PathResult<T>;
}

impl<T> ToPathResult<T> for ServiceResult<T> {
    fn to_path_result(self) -> PathResult<T> {
        match self {
            Ok(data) => Ok(Json(data)),
            Err(app_error) => Err(app_error),
        }
    }
}

pub trait ToPathEmptyResult {
    fn to_path_empty_result(self) -> PathEmptyResult;
}

impl<T> ToPathEmptyResult for ServiceResult<T> {
    fn to_path_empty_result(self) -> PathEmptyResult {
        match self {
            Ok(_) => Ok(()),
            Err(app_error) => Err(app_error),
        }
    }
}

pub enum AppError {
    ValidationError(Box<dyn Fn(&str) -> String + Send>),
    DieselError(DieselError, Option<String>, Option<String>),
    ForbiddenError(String),
    NotFoundError(String),
    InternalServerError,
}

pub trait ToAppError {
    fn to_app_error(self, not_found_key: Option<String>, unique_key: Option<String>) -> AppError;
}

impl ToAppError for DieselError {
    fn to_app_error(self, not_found_key: Option<String>, unique_key: Option<String>) -> AppError {
        AppError::DieselError(self, not_found_key, unique_key)
    }
}

pub trait ToServiceResult<T> {
    fn to_service_result(self) -> ServiceResult<T>;
    fn to_service_result_find(self, not_found_key: String) -> ServiceResult<T>;
    fn to_service_result_unique(self, unique_key: String) -> ServiceResult<T>;
}

impl<T> ToServiceResult<T> for Result<T, DieselError> {
    fn to_service_result(self) -> ServiceResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_app_error(None, None)),
        }
    }
    fn to_service_result_find(self, not_found_key: String) -> ServiceResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_app_error(Some(not_found_key), None)),
        }
    }
    fn to_service_result_unique(self, unique_key: String) -> ServiceResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_app_error(None, Some(unique_key))),
        }
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let locale = request.local_cache::<Locale, _>(|| unreachable!());
        match self {
            AppError::ValidationError(get_message) => {
                let mut response = get_message(&locale.get_locale()).respond_to(request)?;
                response.set_status(Status::BadRequest);
                return Ok(response);
            }
            AppError::DieselError(diesel_error, not_found_key, unique_error_key) => {
                match diesel_error {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        let key = unique_error_key.unwrap().clone();
                        let result: PathEmptyResult = validation_error!(&key);
                        result.respond_to(request)
                    }
                    DieselError::NotFound => {
                        let result: PathEmptyResult = not_found_error!(not_found_key.unwrap());
                        result.respond_to(request)
                    }
                    _ => {
                        let result: PathEmptyResult = internal_server_error!();
                        result.respond_to(request)
                    }
                }
            }
            AppError::ForbiddenError(forbidden_key) => {
                let mut response =
                    t!(&forbidden_key, locale = &locale.get_locale()).respond_to(request)?;
                response.set_status(Status::Forbidden);
                return Ok(response);
            }
            AppError::NotFoundError(not_found_key) => {
                let mut response =
                    t!(&not_found_key, locale = &locale.get_locale()).respond_to(request)?;
                response.set_status(Status::NotFound);
                return Ok(response);
            }
            AppError::InternalServerError => {
                let mut response = t!("internal_server_error", locale = &locale.get_locale())
                    .respond_to(request)?;
                response.set_status(Status::BadRequest);
                return Ok(response);
            }
        }
    }
}

impl OpenApiResponderInner for AppError {
    fn responses(gen: &mut OpenApiGenerator) -> RocketOkapiResult<Responses> {
        <String>::responses(gen)
    }
}

pub fn handle_bad_request_error<'r>(status: Status, request: &'r Request<'_>) -> BoxFuture<'r> {
    Box::pin(async move { get_response(status, request, "bad_request_error").await })
}

pub fn handle_unauthorized_error<'r>(status: Status, request: &'r Request<'_>) -> BoxFuture<'r> {
    Box::pin(async move { get_response(status, request, "unauthorized_error").await })
}

pub fn handle_internal_server_error<'r>(status: Status, request: &'r Request<'_>) -> BoxFuture<'r> {
    Box::pin(async move { get_response(status, request, "internal_server_error").await })
}

async fn get_request_locale<'r>(request: &'r Request<'_>) -> Locale {
    let locale = Locale::new();
    match RocketAcceptLanguage::from_request(request).await {
        Outcome::Success(accept_language) => {
            locale.set_from_accept_language(&AcceptLanguage(accept_language.accept_language));
        }
        _ => locale.set_from_string(String::from("en-US")),
    }
    locale
}

async fn get_response<'r>(
    status: Status,
    request: &'r Request<'_>,
    key: &str,
) -> Result<Response<'r>, Status> {
    let locale = get_request_locale(request).await;
    let message = t!(key, locale = &locale.get_locale());
    let mut response = (status, message).respond_to(request).unwrap();
    response.set_header(Header::new(
        header::CONTENT_LANGUAGE.as_str(),
        locale.get_locale(),
    ));
    Ok(response)
}
