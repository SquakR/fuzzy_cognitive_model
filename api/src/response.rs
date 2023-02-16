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

impl AppError {
    pub fn update_diesel_result<T>(result: Result<T, DieselError>) -> Result<T, AppError> {
        match result {
            Ok(v) => Ok(v),
            Err(err) => Err(AppError::from_diesel_error(err, None)),
        }
    }
    pub fn update_diesel_result_find<T>(
        result: Result<T, DieselError>,
        not_found_key: String,
    ) -> Result<T, AppError> {
        match result {
            Ok(v) => Ok(v),
            Err(err) => Err(AppError::from_diesel_error(err, Some(not_found_key))),
        }
    }
    pub fn from_diesel_error(diesel_error: DieselError, not_found_key: Option<String>) -> AppError {
        AppError::DieselError(diesel_error, not_found_key)
    }
}

pub struct PathResult<T> {
    pub service_result: ServiceResult<T>,
    pub locale: String,
}

impl<T> PathResult<T> {
    pub fn new(service_result: ServiceResult<T>, locale: String) -> PathResult<T> {
        PathResult {
            service_result,
            locale,
        }
    }
}

impl<'r, T: Responder<'r, 'static>> Responder<'r, 'static> for PathResult<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self.service_result {
            Ok(value) => value.respond_to(req),
            Err(app_error) => match app_error {
                AppError::ValidationError(get_message) => {
                    let mut response = get_message(&self.locale).respond_to(req)?;
                    response.set_status(Status::BadRequest);
                    response
                        .set_header(Header::new(header::CONTENT_LANGUAGE.as_str(), self.locale));
                    return Ok(response);
                }
                AppError::DieselError(diesel_error, not_found_key) => match diesel_error {
                    DieselError::NotFound => PathResult::<()>::new(
                        Err(AppError::NotFoundError(not_found_key.unwrap())),
                        self.locale,
                    )
                    .respond_to(req),
                    _ => PathResult::<()>::new(Err(AppError::InternalServerError), self.locale)
                        .respond_to(req),
                },
                AppError::ForbiddenError(forbidden_key) => {
                    let mut response = t!(&forbidden_key, locale = &self.locale).respond_to(req)?;
                    response.set_status(Status::Forbidden);
                    response
                        .set_header(Header::new(header::CONTENT_LANGUAGE.as_str(), self.locale));
                    return Ok(response);
                }
                AppError::NotFoundError(not_found_key) => {
                    let mut response = t!(&not_found_key, locale = &self.locale).respond_to(req)?;
                    response.set_status(Status::NotFound);
                    response
                        .set_header(Header::new(header::CONTENT_LANGUAGE.as_str(), self.locale));
                    return Ok(response);
                }
                AppError::InternalServerError => {
                    let mut response =
                        t!("internal_server_error", locale = &self.locale).respond_to(req)?;
                    response
                        .set_header(Header::new(header::CONTENT_LANGUAGE.as_str(), self.locale));
                    response.set_status(Status::BadRequest);
                    return Ok(response);
                }
            },
        }
    }
}

impl<T> OpenApiResponderInner for PathResult<T> {
    fn responses(gen: &mut OpenApiGenerator) -> RocketOkapiResult<Responses> {
        let ok_responses = <String>::responses(gen)?;
        let err_responses = <Status>::responses(gen)?;
        rocket_okapi::util::produce_any_responses(ok_responses, err_responses)
    }
}
