use crate::db;
use crate::models::User;
use crate::services::session_services;
use crate::services::users_services;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::Result as RocketOkapiResult;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let connection = &mut db::establish_connection();
        match request.cookies().get_private("session_id") {
            Some(session_id) => {
                let session_id = match session_id.value().parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => return Outcome::Failure((Status::BadRequest, ())),
                };
                let session = match session_services::find_session_by_id(connection, session_id) {
                    Ok(value) => value,
                    Err(_) => return Outcome::Failure((Status::BadRequest, ())),
                };
                if !session.is_active {
                    return Outcome::Failure((Status::BadRequest, ()));
                }
                Outcome::Success(users_services::find_user_by_session(connection, &session))
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for User {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
