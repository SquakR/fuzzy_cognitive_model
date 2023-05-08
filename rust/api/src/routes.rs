mod model_routes;
mod project_routes;
mod user_routes;
use crate::plugins::adjustment::routes as adjustment_routes;
use crate::plugins::concept_constraints::routes as concept_constraints_routes;
use crate::plugins::connection_constraints::routes as connection_constraints_routes;
use crate::plugins::control_concepts::routes as control_concepts_routes;
use crate::plugins::control_connections::routes as control_connections_routes;
use crate::plugins::target_concepts::routes as target_concepts_routes;
use okapi::openapi3::{Object, OpenApi, Parameter, ParameterValue, RefOr, SchemaObject};
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

macro_rules! add_accept_language_header {
    ($path:expr, $operation:ident) => {
        if let Some(get) = &mut $path.$operation {
            get.parameters.push(RefOr::Object(Parameter {
                name: "Accept-Language".to_owned(),
                location: "header".to_owned(),
                description: None,
                required: true,
                deprecated: false,
                allow_empty_value: false,
                value: ParameterValue::Schema {
                    style: None,
                    explode: None,
                    allow_reserved: false,
                    schema: SchemaObject::default(),
                    example: None,
                    examples: None,
                },
                extensions: Object::default(),
            }))
        }
    };
}

pub fn add_accept_language_header(spec: &mut OpenApi) -> () {
    for (_, path) in spec.paths.iter_mut() {
        add_accept_language_header!(path, get);
        add_accept_language_header!(path, post);
        add_accept_language_header!(path, put);
        add_accept_language_header!(path, patch);
        add_accept_language_header!(path, delete);
    }
}

macro_rules! get_routes {
    ($first_route:expr, $($route:expr),*) => {{
        let settings = OpenApiSettings::new();
        let mut spec = openapi_spec![$first_route $(,$route)*](&settings);
        spec.info.title = String::from("Fuzzy Cognitive Model");
        patch_wrong_content_type(&mut spec, "/user", Operation::Post);
        patch_wrong_content_type(&mut spec, "/me", Operation::Put);
        add_accept_language_header(&mut spec);
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
                user_routes::create_user,
                user_routes::get_users,
                user_routes::confirm_email,
                user_routes::get_me,
                user_routes::change_me,
                user_routes::change_me_locale,
                user_routes::change_me_password,
                user_routes::request_password_reset,
                user_routes::reset_password,
                user_routes::sign_in,
                user_routes::sign_out_multiple,
                user_routes::sign_out,
                user_routes::get_sessions,
                user_routes::get_user_avatar,
                project_routes::create_project,
                project_routes::get_projects,
                project_routes::get_plugins,
                project_routes::get_permissions,
                project_routes::get_project_users,
                project_routes::change_project,
                project_routes::set_project_plugins,
                project_routes::set_project_user_permissions,
                project_routes::invite_user,
                project_routes::cancel_invitation,
                project_routes::respond_to_invitation,
                project_routes::leave_project,
                project_routes::exclude_user,
                project_routes::delete_project,
                model_routes::get_model,
                model_routes::get_model_copy,
                model_routes::get_active_users,
                model_routes::create_concept,
                model_routes::change_concept_description,
                model_routes::change_concept_value,
                model_routes::move_concept,
                model_routes::delete_concept,
                model_routes::create_connection,
                model_routes::change_connection_description,
                model_routes::change_connection_value,
                model_routes::delete_connection,
                control_concepts_routes::change_concept_is_control,
                target_concepts_routes::change_target_concept,
                control_connections_routes::change_connection_is_control,
                concept_constraints_routes::change_concept_constraint,
                connection_constraints_routes::change_connection_constraint,
                adjustment_routes::change_dynamic_model_type,
                adjustment_routes::adjust,
                adjustment_routes::get_adjustment_runs,
                adjustment_routes::get_adjustment_generations,
                adjustment_routes::get_adjustment_chromosomes
            ),
        )
    }
}
