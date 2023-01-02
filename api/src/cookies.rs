use rocket::http::{Cookie, CookieJar};

pub fn add_session_id(cookies_jar: &CookieJar<'_>, session_id: i32) -> () {
    cookies_jar.add_private(
        Cookie::build("session_id", session_id.to_string())
            .http_only(true)
            .finish(),
    );
}

pub fn get_session_id(cookies_jar: &CookieJar<'_>) -> Option<i32> {
    match cookies_jar.get_private("session_id") {
        Some(session_id) => match session_id.value().parse::<i32>() {
            Ok(session_id) => Some(session_id),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn has_session_id(cookies_jar: &CookieJar<'_>) -> bool {
    get_session_id(cookies_jar).is_some()
}

pub fn remove_session_id(cookies_jar: &CookieJar<'_>) -> () {
    cookies_jar.remove_private(Cookie::named("session_id"))
}
