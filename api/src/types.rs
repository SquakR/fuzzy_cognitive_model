use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Metadata, ObjectValidation, Schema, SchemaObject};
use schemars::{Map, Set};

/// Type of user (expert or researcher)
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserOutType {
    /// User identifier
    pub id: i32,
    /// User nickname
    pub username: String,
    /// User email
    pub email: String,
    /// Is user email confirmed
    pub is_email_confirmed: bool,
    /// User name
    pub first_name: String,
    /// User second name or patronymic
    pub second_name: Option<String>,
    /// User last name
    pub last_name: String,
    /// User avatar
    pub avatar: Option<String>,
    /// User preferred language
    pub language: Option<String>,
    /// User creation time
    pub created_at: DateTime<Utc>,
    /// User update time
    pub updated_at: DateTime<Utc>,
}

/// Type of user (expert or researcher) to create
#[derive(FromForm)]
pub struct UserInCreateType<'r> {
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
    /// User preferred language
    pub language: Option<String>,
}

/// Type of user (expert or researcher) to change
#[derive(FromForm)]
pub struct UserInChangeType<'r> {
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
pub struct DeviceType {
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
pub struct OSType {
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
pub struct ProductType {
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
pub struct SessionType {
    /// Session identifier
    pub id: i32,
    /// Is session current
    pub is_current: bool,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// User ip address
    pub ip_address: String,
    /// User device
    pub device: DeviceType,
    /// User operation system
    pub os: OSType,
    /// User product
    pub product: ProductType,
}

/// User credentials
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CredentialsType {
    /// User nickname
    pub username: String,
    /// User password
    pub password: String,
}

/// Change user preferred language
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLanguageType {
    /// New user preferred language
    pub language: Option<String>,
}

/// Change user password type
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordType {
    /// Old user password
    pub old_password: String,
    /// New user password
    pub new_password: String,
}

/// Reset user password type
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordType {
    /// Reset password token
    pub token: String,
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

impl<'r> JsonSchema for UserInCreateType<'r> {
    fn schema_name() -> String {
        String::from("UserInCreateType")
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
                (
                    "language",
                    InstanceType::String,
                    "User preferred language",
                    false,
                    None
                )
            ],
            "Type of user (expert or researcher) to create"
        )
    }
}

impl<'r> JsonSchema for UserInChangeType<'r> {
    fn schema_name() -> String {
        String::from("UserInUpdateType")
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
