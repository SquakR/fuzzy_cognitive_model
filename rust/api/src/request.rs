use crate::authenticate;
use crate::db;
use crate::locale::Locale;
use crate::models::User;
use crate::plugins::Plugins;
use crate::web_socket::{WebSocketAdjustmentRunService, WebSocketModelService};
use chrono::{DateTime, Utc};
use rocket::form::{self, FromFormField, ValueField};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_accept_language::{AcceptLanguage as RocketAcceptLanguage, LanguageIdentifier};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::Result as RocketOkapiResult;
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct DateTimeWrapper(pub DateTime<Utc>);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for DateTimeWrapper {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match DateTime::parse_from_rfc3339(field.value) {
            Ok(date_time) => Ok(DateTimeWrapper(date_time.into())),
            Err(_) => Err(form::Error::validation("Invalid date"))?,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let accept_language = match request.guard::<&AcceptLanguage>().await {
            Outcome::Success(accept_language) => accept_language,
            Outcome::Failure(failure) => return Outcome::Failure(failure),
            Outcome::Forward(forward) => return Outcome::Forward(forward),
        };
        let authentication_result = request.local_cache(|| {
            let conn = &mut db::establish_connection();
            authenticate!(conn, request.cookies())
        });
        match authentication_result {
            Ok((user, _)) => {
                let locale = request.guard::<&Locale>().await.unwrap();
                locale.set_from_user(user, accept_language);
                Outcome::Success(user.clone())
            }
            Err(status) => Outcome::Failure((status.clone(), ())),
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
impl<'r> FromRequest<'r> for &'r AcceptLanguage {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<&RocketAcceptLanguage>().await {
            Outcome::Success(accept_language) => Outcome::Success(
                request.local_cache(|| AcceptLanguage(accept_language.accept_language.clone())),
            ),
            Outcome::Failure(failure) => Outcome::Failure(failure),
            Outcome::Forward(forward) => Outcome::Forward(forward),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r AcceptLanguage {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Locale {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<&AcceptLanguage>().await {
            Outcome::Success(accept_language) => Outcome::Success(request.local_cache(|| {
                let locale = Locale::new();
                locale.set_from_accept_language(accept_language);
                locale
            })),
            Outcome::Failure(failure) => Outcome::Failure(failure),
            Outcome::Forward(forward) => Outcome::Forward(forward),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r Locale {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WebSocketModelService {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let model_service = request.local_cache::<WebSocketModelService, _>(|| unreachable!());
        Outcome::Success(model_service.clone())
    }
}

impl<'r> OpenApiFromRequest<'r> for WebSocketModelService {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WebSocketAdjustmentRunService {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let adjustment_run_service =
            request.local_cache::<WebSocketAdjustmentRunService, _>(|| unreachable!());
        Outcome::Success(adjustment_run_service.clone())
    }
}

impl<'r> OpenApiFromRequest<'r> for WebSocketAdjustmentRunService {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Plugins {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let plugins_emitter = request.local_cache::<Plugins, _>(|| unreachable!());
        Outcome::Success(plugins_emitter)
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r Plugins {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
