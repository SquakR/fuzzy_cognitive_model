use crate::authenticate;
use crate::db;
use crate::models::User;
use crate::web_socket::WebSocketProjectService;
use chrono::{DateTime, Utc};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::{self, FromFormField, ValueField};
use rocket::http::hyper::header;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::{Data, Request, Response};
use rocket_accept_language::{AcceptLanguage as RocketAcceptLanguage, LanguageIdentifier};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::Result as RocketOkapiResult;
use schemars::JsonSchema;
use std::sync::Mutex;

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

const AVAILABLE_LOCALES: [(&str, &str); 4] = [
    ("en-US", "en-US"),
    ("en", "en-US"),
    ("ru-RU", "ru-RU"),
    ("ru", "ru-RU"),
];

pub struct Locale {
    pub locale: Mutex<Option<String>>,
}

impl Locale {
    pub fn new() -> Self {
        Self {
            locale: Mutex::new(None),
        }
    }
    pub fn get_locale(&self) -> String {
        self.locale.lock().unwrap().to_owned().unwrap()
    }
    pub fn set_from_string(&self, value: String) -> () {
        *self.locale.lock().unwrap() = Some(value);
    }
    pub fn set_from_accept_language(&self, accept_language: &AcceptLanguage) -> () {
        for identifier_locale in accept_language.0.iter().map(|i| i.to_string()) {
            for available_locale in AVAILABLE_LOCALES {
                if identifier_locale == available_locale.0 {
                    *self.locale.lock().unwrap() = Some(available_locale.1.to_owned());
                    return;
                }
            }
        }
        *self.locale.lock().unwrap() = Some(String::from("en-US"));
    }
    pub fn set_from_user(&self, user: &User, accept_language: &AcceptLanguage) -> () {
        if let Some(language) = &user.language {
            for available_locale in AVAILABLE_LOCALES {
                if language == available_locale.0 {
                    *self.locale.lock().unwrap() = Some(available_locale.1.to_owned());
                    return;
                }
            }
        }
        for identifier_locale in accept_language.0.iter().map(|i| i.to_string()) {
            for available_locale in AVAILABLE_LOCALES {
                if identifier_locale == available_locale.0 {
                    *self.locale.lock().unwrap() = Some(available_locale.1.to_owned());
                    return;
                }
            }
        }
        *self.locale.lock().unwrap() = Some(String::from("en-US"));
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

pub struct LocaleFairing;

#[rocket::async_trait]
impl Fairing for LocaleFairing {
    fn info(&self) -> Info {
        Info {
            name: "Locale Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let _locale = request.guard::<&Locale>().await.unwrap();
    }
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let locale = request.local_cache::<Locale, _>(|| unreachable!());
        response.set_header(Header::new(
            header::CONTENT_LANGUAGE.as_str(),
            locale.get_locale(),
        ));
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WebSocketProjectService {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let project_service = request.local_cache::<WebSocketProjectService, _>(|| unreachable!());
        Outcome::Success(project_service.clone())
    }
}

impl<'r> OpenApiFromRequest<'r> for WebSocketProjectService {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> RocketOkapiResult<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
