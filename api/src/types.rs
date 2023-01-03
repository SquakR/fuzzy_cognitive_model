use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::serde::Deserialize;
use rocket_okapi::JsonSchema;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Metadata, ObjectValidation, Schema, SchemaObject};
use schemars::{Map, Set};

/// Fuzzy cognitive model user (expert or researcher)
#[derive(FromForm)]
pub struct UserIn<'r> {
    /// User nickname
    pub username: String,
    /// User password
    pub password: String,
    /// User email
    pub email: String,
    /// User first name
    #[field(name = "firstName")]
    pub first_name: String,
    /// User second name or patronymic
    #[field(name = "secondName")]
    pub second_name: Option<String>,
    /// User last name
    #[field(name = "lastName")]
    pub last_name: String,
    /// User avatar
    pub avatar: Option<TempFile<'r>>,
}

/// User credentials
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    /// User nickname
    pub username: String,
    /// User password
    pub password: String,
}

impl<'r> JsonSchema for UserIn<'r> {
    fn schema_name() -> String {
        return String::from("UserIn");
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        let mut properties = Map::new();
        let mut required = Set::new();
        for (name, description, req, format) in [
            ("username", "User nickname", true, None),
            ("password", "User password", true, None),
            ("email", "User email", true, None),
            ("firstName", "User first name", true, None),
            ("secondName", "User second name or patronymic", false, None),
            ("lastName", "User last name", true, None),
            ("avatar", "User avatar", false, Some(String::from("binary"))),
        ] {
            properties.insert(
                String::from(name),
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    format,
                    metadata: Some(Box::new(Metadata {
                        description: Some(String::from(description)),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
                .into(),
            );
            if req {
                required.insert(String::from(name));
            }
        }

        SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            metadata: Some(Box::new(Metadata {
                description: Some(String::from(
                    "Fuzzy cognitive model user (expert or researcher)",
                )),
                ..Default::default()
            })),
            object: Some(Box::new(ObjectValidation {
                properties,
                required,
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}
