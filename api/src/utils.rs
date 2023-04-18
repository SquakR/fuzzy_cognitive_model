use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

pub fn get_env(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}

pub fn get_jwt_key() -> Hmac<Sha256> {
    let secret_key = get_env("ROCKET_SECRET_KEY");
    Hmac::new_from_slice(secret_key.as_bytes()).unwrap()
}
