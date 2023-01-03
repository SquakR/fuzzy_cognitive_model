use diesel::result::Error as DieselError;
use okapi::openapi3::Responses;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::Result as RocketOkapiResult;

pub enum AppError {
    BadRequestError,
    ValidationError(String),
    DieselError(DieselError),
    InternalServerError,
}

impl AppError {
    pub fn from_diesel_error(diesel_error: DieselError) -> AppError {
        AppError::DieselError(diesel_error)
    }
    pub fn update_result<T>(result: Result<T, DieselError>) -> Result<T, AppError> {
        match result {
            Ok(v) => Ok(v),
            Err(err) => Err(AppError::from_diesel_error(err)),
        }
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match &self {
            AppError::BadRequestError => Status::BadRequest.respond_to(req),
            AppError::ValidationError(validation_error) => {
                let mut response = validation_error.to_owned().respond_to(req)?;
                response.set_status(Status::BadRequest);
                return Ok(response);
            }
            AppError::DieselError(diesel_error) => match diesel_error {
                DieselError::NotFound => Status::NotFound.respond_to(req),
                _ => Status::InternalServerError.respond_to(req),
            },
            AppError::InternalServerError => Status::InternalServerError.respond_to(req),
        }
    }
}

impl OpenApiResponderInner for AppError {
    fn responses(gen: &mut OpenApiGenerator) -> RocketOkapiResult<Responses> {
        let ok_responses = <String>::responses(gen)?;
        let err_responses = <Status>::responses(gen)?;
        rocket_okapi::util::produce_any_responses(ok_responses, err_responses)
    }
}
