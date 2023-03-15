use crate::authenticate;
use crate::db;
use crate::models::User;
use chrono::{DateTime, Utc};
use okapi::openapi3::{Object, Parameter, ParameterValue};
use rocket::form::{self, FromFormField, ValueField};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
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
        let authentication_result = request.local_cache(|| {
            let conn = &mut db::establish_connection();
            authenticate!(conn, request.cookies())
        });
        match authentication_result {
            Ok((user, _)) => Outcome::Success(user.clone()),
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
        gen: &mut OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        add_accept_language_header(gen, required)
    }
}

pub trait BaseLocale {
    fn get_locale(&self) -> &str;
}

pub struct Locale(pub String);

impl Locale {
    pub fn new(accept_language: &AcceptLanguage) -> Locale {
        let available_locales = [
            ("en-US", "en-US"),
            ("en", "en-US"),
            ("ru-RU", "ru-RU"),
            ("ru", "ru-RU"),
        ];
        let mut locale = String::from("en-US");
        'outer: for identifier_locale in accept_language.0.iter().map(|i| i.to_string()) {
            for available_locale in available_locales {
                if identifier_locale == available_locale.0 {
                    locale = available_locale.1.to_owned();
                    break 'outer;
                }
            }
        }
        Locale(locale)
    }
}

impl BaseLocale for Locale {
    fn get_locale(&self) -> &str {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Locale {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<&AcceptLanguage>().await {
            Outcome::Success(accept_language) => Outcome::Success(Locale::new(accept_language)),
            Outcome::Failure(failure) => Outcome::Failure(failure),
            Outcome::Forward(forward) => Outcome::Forward(forward),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for Locale {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        add_accept_language_header(gen, required)
    }
}

pub struct UserLocale(pub String);

impl UserLocale {
    pub fn new(user: &User, accept_language: &AcceptLanguage) -> UserLocale {
        let available_locales = [
            ("en-US", "en-US"),
            ("en", "en-US"),
            ("ru-RU", "ru-RU"),
            ("ru", "ru-RU"),
        ];
        let mut locale = String::from("en-US");
        if let Some(language) = &user.language {
            locale = language.to_owned();
        } else {
            'outer: for identifier_locale in accept_language.0.iter().map(|i| i.to_string()) {
                for available_locale in available_locales {
                    if identifier_locale == available_locale.0 {
                        locale = available_locale.1.to_owned();
                        break 'outer;
                    }
                }
            }
        }
        UserLocale(locale)
    }
}

impl BaseLocale for UserLocale {
    fn get_locale(&self) -> &str {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserLocale {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match request.guard::<User>().await {
            Outcome::Success(user) => user,
            Outcome::Failure(failure) => return Outcome::Failure(failure),
            Outcome::Forward(forward) => return Outcome::Forward(forward),
        };
        let accept_language = match request.guard::<&AcceptLanguage>().await {
            Outcome::Success(accept_language) => accept_language,
            Outcome::Failure(failure) => return Outcome::Failure(failure),
            Outcome::Forward(forward) => return Outcome::Forward(forward),
        };
        Outcome::Success(UserLocale::new(&user, &accept_language))
    }
}

impl<'r> OpenApiFromRequest<'r> for UserLocale {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        add_accept_language_header(gen, required)
    }
}

fn add_accept_language_header(
    gen: &mut OpenApiGenerator,
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
