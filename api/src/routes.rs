mod user_routes;
use okapi::openapi3::{OpenApi, RefOr};
use rocket::{Build, Rocket};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi_routes, openapi_spec};

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

macro_rules! get_routes {
    ($first_route:expr, $($route:expr),*) => {{
        let settings = OpenApiSettings::new();
        let mut spec = openapi_spec![$first_route $(,$route)*](&settings);
        spec.info.title = String::from("Fuzzy Cognitive Model");
        patch_wrong_content_type(&mut spec, "/user", Operation::Post);
        patch_wrong_content_type(&mut spec, "/me", Operation::Put);
        openapi_routes![$first_route $(,$route)*](Some(spec), &settings)
    }};
}

pub trait MountRoutes {
    fn mount_routes(self, base: &str) -> Self;
}

impl MountRoutes for Rocket<Build> {
    fn mount_routes(self, base: &str) -> Self {
        self.mount(
            base,
            get_routes!(
                user_routes::get_users,
                user_routes::create_user,
                user_routes::confirm_email,
                user_routes::get_me,
                user_routes::change_me,
                user_routes::change_me_language,
                user_routes::change_me_password,
                user_routes::request_password_reset,
                user_routes::reset_password,
                user_routes::sign_in,
                user_routes::sign_out_multiple,
                user_routes::sign_out,
                user_routes::get_sessions,
                user_routes::get_user_avatar
            ),
        )
    }
}