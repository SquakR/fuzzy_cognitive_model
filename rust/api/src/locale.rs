use crate::models::User;
use crate::request::AcceptLanguage;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::hyper::header;
use rocket::http::Header;
use rocket::{Data, Request, Response};
use std::sync::Mutex;

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
        if let Some(locale) = &user.locale {
            for available_locale in AVAILABLE_LOCALES {
                if locale == available_locale.0 {
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
