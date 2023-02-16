use crate::models::User;
use crate::request_guards::AcceptLanguage;

pub fn get_locale(user: Option<&User>, accept_language: &AcceptLanguage) -> String {
    let available_locales = [
        ("en-US", "en-US"),
        ("en", "en-US"),
        ("ru-RU", "ru-RU"),
        ("ru", "ru-RU"),
    ];
    let mut locale = String::from("en-US");
    if let Some(user) = user {
        if let Some(language) = &user.language {
            locale = language.to_owned();
        }
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
    locale
}
