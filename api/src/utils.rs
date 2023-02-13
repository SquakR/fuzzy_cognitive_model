use hmac::{Hmac, Mac};
use okapi::openapi3::{OpenApi, RefOr};
use sha2::Sha256;
use std::env;

pub fn get_env(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}

pub enum Operation {
    Post,
    Put,
}

pub fn patch_wrong_content_type(spec: &mut OpenApi, key: &str, operation: Operation) -> () {
    let path = spec.paths.get_mut(key).unwrap();
    let operation = match operation {
        Operation::Post => path.post.as_mut(),
        Operation::Put => path.put.as_mut(),
    };
    let rb = operation.unwrap().request_body.as_mut().unwrap();
    match rb {
        RefOr::Object(obj) => {
            let schema = obj.content.remove("application/octet-stream").unwrap();
            obj.content
                .insert(String::from("multipart/form-data"), schema);
        }
        _ => unreachable!(),
    }
}

pub fn get_jwt_key() -> Hmac<Sha256> {
    let secret_key = get_env("SECRET_KEY");
    Hmac::new_from_slice(secret_key.as_bytes()).unwrap()
}
