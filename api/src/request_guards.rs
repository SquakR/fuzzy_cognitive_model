use crate::cookies;
use crate::db;
use crate::models::User;
use crate::services::session_services;
use crate::services::users_services;
use okapi::openapi3::{Object, Parameter, ParameterValue};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_accept_language::{AcceptLanguage as RocketAcceptLanguage, LanguageIdentifier};
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
            let _sessions = session_services::deactivate_all_user_sessions(connection, user.id);
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

pub struct UserAgent(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("User-Agent") {
            Some(user_agent) => Outcome::Success(UserAgent(user_agent.to_owned())),
            None => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for UserAgent {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

pub struct AcceptLanguage(pub Vec<LanguageIdentifier>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AcceptLanguage {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let result = RocketAcceptLanguage::from_request(request).await;
        match result {
            Outcome::Success(accept_language) => {
                Outcome::Success(AcceptLanguage(accept_language.accept_language))
            }
            Outcome::Forward(forward) => Outcome::Forward(forward),
            Outcome::Failure(failure) => Outcome::Failure(failure),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for AcceptLanguage {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "Accept-Language".to_owned(),
            location: "header".to_owned(),
            description: None,
            required,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Object::default(),
        }))
    }
}
