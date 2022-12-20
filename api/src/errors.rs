use diesel::result::Error as DieselError;
use okapi::openapi3::Responses;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::Result as RocketOkapiResult;

pub struct QueryError {
    diesel_error: DieselError,
}

impl QueryError {
    pub fn new(diesel_error: DieselError) -> QueryError {
        QueryError { diesel_error }
    }
    pub fn update_result<T>(result: Result<T, DieselError>) -> Result<T, QueryError> {
        match result {
            Ok(v) => Ok(v),
            Err(err) => Err(QueryError::new(err)),
        }
    }
}

impl<'r> Responder<'r, 'static> for QueryError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self.diesel_error {
            DieselError::NotFound => Status::NotFound.respond_to(req),
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}

impl OpenApiResponderInner for QueryError {
    fn responses(gen: &mut OpenApiGenerator) -> RocketOkapiResult<Responses> {
        <Status>::responses(gen)
    }
}
