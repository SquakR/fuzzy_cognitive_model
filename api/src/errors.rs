use diesel::result::Error as DieselError;
use okapi::openapi3::Responses;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::JsonSchema;
use rocket_okapi::Result as RocketOkapiResult;

/// Error in the business logic of the application
#[derive(Serialize, JsonSchema)]
pub struct BusinessLogicError {
    /// Type of business logic error
    #[serde(rename = "type")]
    r#type: String,
    /// Message of business logic error
    message: String,
}

pub enum AppError {
    BusinessLogicError(BusinessLogicError),
    DieselError(DieselError),
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
            AppError::BusinessLogicError(business_logic_error) => {
                Json(business_logic_error).respond_to(req)
            }
            AppError::DieselError(diesel_error) => match diesel_error {
                DieselError::NotFound => Status::NotFound.respond_to(req),
                _ => Status::InternalServerError.respond_to(req),
            },
        }
    }
}

impl OpenApiResponderInner for AppError {
    fn responses(gen: &mut OpenApiGenerator) -> RocketOkapiResult<Responses> {
        let ok_responses = <Json<BusinessLogicError>>::responses(gen)?;
        let err_responses = <Status>::responses(gen)?;
        rocket_okapi::util::produce_any_responses(ok_responses, err_responses)
    }
}
