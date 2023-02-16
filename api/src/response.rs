use crate::request_guards::{BaseLocale, Locale};
use diesel::result::Error as DieselError;
use okapi::openapi3::Responses;
use rocket::http::hyper::header;
use rocket::http::Header;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::Result as RocketOkapiResult;
use rust_i18n::t;

pub type ServiceResult<T> = Result<T, AppError>;

pub enum AppError {
    ValidationError(Box<dyn Fn(&str) -> String>),
    DieselError(DieselError, Option<String>),
    ForbiddenError(String),
    NotFoundError(String),
    InternalServerError,
}

pub trait ToAppError {
    fn to_app_error(self, not_found_key: Option<String>) -> AppError;
}

impl ToAppError for DieselError {
    fn to_app_error(self, not_found_key: Option<String>) -> AppError {
        AppError::DieselError(self, not_found_key)
    }
}

pub trait ToServiceResult<T> {
    fn to_service_result(self) -> ServiceResult<T>;
    fn to_service_result_find(self, not_found_key: String) -> ServiceResult<T>;
}

impl<T> ToServiceResult<T> for Result<T, DieselError> {
    fn to_service_result(self) -> ServiceResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_app_error(None)),
        }
    }
    fn to_service_result_find(self, not_found_key: String) -> ServiceResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_app_error(Some(not_found_key))),
        }
    }
}

pub struct PathResult<T, L: BaseLocale> {
    pub service_result: ServiceResult<T>,
    pub locale: L,
}

impl<T, L: BaseLocale> PathResult<T, L> {
    pub fn new(service_result: ServiceResult<T>, locale: L) -> PathResult<T, L> {
        PathResult {
            service_result,
            locale,
        }
    }
}

impl<'r, R: Responder<'r, 'static>, L: BaseLocale> Responder<'r, 'static> for PathResult<R, L> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self.service_result {
            Ok(value) => value.respond_to(req),
            Err(app_error) => match app_error {
                AppError::ValidationError(get_message) => {
                    let mut response = get_message(self.locale.get_locale()).respond_to(req)?;
                    response.set_status(Status::BadRequest);
                    response.set_header(Header::new(
                        header::CONTENT_LANGUAGE.as_str(),
                        self.locale.get_locale().to_owned(),
                    ));
                    return Ok(response);
                }
                AppError::DieselError(diesel_error, not_found_key) => match diesel_error {
                    DieselError::NotFound => PathResult::<(), Locale>::new(
                        Err(AppError::NotFoundError(not_found_key.unwrap())),
                        Locale(self.locale.get_locale().to_owned()),
                    )
                    .respond_to(req),
                    _ => PathResult::<(), Locale>::new(
                        Err(AppError::InternalServerError),
                        Locale(self.locale.get_locale().to_owned()),
                    )
                    .respond_to(req),
                },
                AppError::ForbiddenError(forbidden_key) => {
                    let mut response =
                        t!(&forbidden_key, locale = self.locale.get_locale()).respond_to(req)?;
                    response.set_status(Status::Forbidden);
                    response.set_header(Header::new(
                        header::CONTENT_LANGUAGE.as_str(),
                        self.locale.get_locale().to_owned(),
                    ));
                    return Ok(response);
                }
                AppError::NotFoundError(not_found_key) => {
                    let mut response =
                        t!(&not_found_key, locale = self.locale.get_locale()).respond_to(req)?;
                    response.set_status(Status::NotFound);
                    response.set_header(Header::new(
                        header::CONTENT_LANGUAGE.as_str(),
                        self.locale.get_locale().to_owned(),
                    ));
                    return Ok(response);
                }
                AppError::InternalServerError => {
                    let mut response =
                        t!("internal_server_error", locale = self.locale.get_locale())
                            .respond_to(req)?;
                    response.set_header(Header::new(
                        header::CONTENT_LANGUAGE.as_str(),
                        self.locale.get_locale().to_owned(),
                    ));
                    response.set_status(Status::BadRequest);
                    return Ok(response);
                }
            },
        }
    }
}

impl<T, L: BaseLocale> OpenApiResponderInner for PathResult<T, L> {
    fn responses(gen: &mut OpenApiGenerator) -> RocketOkapiResult<Responses> {
        let ok_responses = <String>::responses(gen)?;
        let err_responses = <Status>::responses(gen)?;
        rocket_okapi::util::produce_any_responses(ok_responses, err_responses)
    }
}
