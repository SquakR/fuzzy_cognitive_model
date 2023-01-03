use okapi::openapi3::{OpenApi, RefOr};
use std::env;

pub fn get_env(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}

pub fn patch_wrong_content_type(spec: &mut OpenApi, key: &str) -> () {
    let rb = spec
        .paths
        .get_mut(key)
        .unwrap()
        .post
        .as_mut()
        .unwrap()
        .request_body
        .as_mut()
        .unwrap();
    match rb {
        RefOr::Object(obj) => {
            let schema = obj.content.remove("application/octet-stream").unwrap();
            obj.content
                .insert(String::from("multipart/form-data"), schema);
        }
        _ => unreachable!(),
    }
}
