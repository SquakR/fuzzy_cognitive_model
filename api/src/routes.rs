mod users_routes;
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
                users_routes::create_user,
                users_routes::confirm_email,
                users_routes::get_me,
                users_routes::change_me,
                users_routes::change_me_language,
                users_routes::change_me_password,
                users_routes::request_password_reset,
                users_routes::reset_password,
                users_routes::sign_in,
                users_routes::sign_out_multiple,
                users_routes::sign_out,
                users_routes::get_sessions,
                users_routes::get_user_avatar
            ),
        )
    }
}
