use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Metadata, ObjectValidation, Schema, SchemaObject};
use schemars::{Map, Set};

/// Type of user (expert or researcher) to create
#[derive(FromForm)]
pub struct UserInCreate<'r> {
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

/// Type of user (expert or researcher) to change
#[derive(FromForm)]
pub struct UserInChange<'r> {
    /// User nickname
    pub username: String,
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
    /// Reset user avatar
    pub reset_avatar: bool,
}

/// User device
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Device name
    pub name: Option<String>,
    /// Device brand
    pub brand: Option<String>,
    /// Device model
    pub model: Option<String>,
}

/// User operation system
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OS {
    /// Operation system name
    pub name: Option<String>,
    /// Operation system major version
    pub major: Option<String>,
    /// Operation system minor version
    pub minor: Option<String>,
    /// Operation system patch version
    pub patch: Option<String>,
    /// Operation system patch minor version
    pub patch_minor: Option<String>,
}

/// User product
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// Product name
    pub name: Option<String>,
    /// Product major version
    pub major: Option<String>,
    /// Product minor version
    pub minor: Option<String>,
    /// Product patch version
    pub patch: Option<String>,
}

/// User session
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// Session identifier
    pub id: i32,
    /// Is session current
    pub is_current: bool,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// User ip address
    pub ip_address: String,
    /// User device
    pub device: Device,
    /// User operation system
    pub os: OS,
    /// User product
    pub product: Product,
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

/// Change password type
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    /// Old user password
    pub old_password: String,
    /// New user password
    pub new_password: String,
}

macro_rules! user_json_schema {
    ($properties:expr, $description:expr) => {{
        let mut properties = Map::new();
        let mut required = Set::new();
        for (name, instance_type, description, req, format) in $properties {
            properties.insert(
                String::from(name),
                SchemaObject {
                    instance_type: Some(instance_type.into()),
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
                description: Some(String::from($description)),
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
    }};
}

impl<'r> JsonSchema for UserInCreate<'r> {
    fn schema_name() -> String {
        String::from("UserInCreate")
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        user_json_schema!(
            [
                (
                    "username",
                    InstanceType::String,
                    "User nickname",
                    true,
                    None
                ),
                (
                    "password",
                    InstanceType::String,
                    "User password",
                    true,
                    None
                ),
                ("email", InstanceType::String, "User email", true, None),
                (
                    "firstName",
                    InstanceType::String,
                    "User first name",
                    true,
                    None
                ),
                (
                    "secondName",
                    InstanceType::String,
                    "User second name or patronymic",
                    false,
                    None
                ),
                (
                    "lastName",
                    InstanceType::String,
                    "User last name",
                    true,
                    None
                ),
                (
                    "avatar",
                    InstanceType::String,
                    "User avatar",
                    false,
                    Some(String::from("binary"))
                ),
            ],
            "Type of user (expert or researcher) to create"
        )
    }
}

impl<'r> JsonSchema for UserInChange<'r> {
    fn schema_name() -> String {
        String::from("UserInUpdate")
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        user_json_schema!(
            [
                (
                    "username",
                    InstanceType::String,
                    "User nickname",
                    true,
                    None
                ),
                ("email", InstanceType::String, "User email", true, None),
                (
                    "firstName",
                    InstanceType::String,
                    "User first name",
                    true,
                    None
                ),
                (
                    "secondName",
                    InstanceType::String,
                    "User second name or patronymic",
                    false,
                    None
                ),
                (
                    "lastName",
                    InstanceType::String,
                    "User last name",
                    true,
                    None
                ),
                (
                    "avatar",
                    InstanceType::String,
                    "User avatar",
                    false,
                    Some(String::from("binary"))
                ),
                (
                    "reset_avatar",
                    InstanceType::Boolean,
                    "Reset user avatar",
                    true,
                    None
                )
            ],
            "Type of user (expert or researcher) to change"
        )
    }
}
