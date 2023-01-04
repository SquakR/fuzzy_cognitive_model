use crate::cookies;
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
        let session_id = match cookies::get_session_id(request.cookies()) {
            Some(session_id) => session_id,
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };
        let session = match session_services::find_session_by_id(connection, session_id) {
            Ok(value) => value,
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
        };
        let user = users_services::find_user_by_session(connection, &session);
        if !session.is_active {
            let _sessions = session_services::deactivate_all_user_sessions(connection, &user);
            return Outcome::Failure((Status::BadRequest, ()));
        }
        Outcome::Success(user)
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
