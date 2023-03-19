use crate::models::{ArcValueType, NodeValueType, ProjectUserStatusValue};
use crate::request::DateTimeWrapper;
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

/// Type of project
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOutType {
    /// Project identifier
    pub id: i32,
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project creator
    pub creator: UserOutType,
    /// Is project public
    pub is_public: bool,
    /// Is project archived
    pub is_archived: bool,
    /// Project creation time
    pub created_at: DateTime<Utc>,
    /// Project update time
    pub updated_at: DateTime<Utc>,
    /// Project node value type
    pub node_value_type: NodeValueType,
    /// Project arc value type
    pub arc_value_type: ArcValueType,
    /// Project plugins
    pub plugins: Vec<String>,
}

/// Type of project to create or change
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInType {
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Is project public
    pub is_public: bool,
    /// Is project archived
    pub is_archived: bool,
    /// Project node value type
    pub node_value_type: NodeValueType,
    /// Project arc value type
    pub arc_value_type: ArcValueType,
}

#[derive(JsonSchema, FromFormField)]
/// Type of project group filter
pub enum ProjectGroupFilterType {
    /// Show only public projects
    Public,
    /// Show only user projects
    Private,
    /// Show public and user projects
    Both,
}

/// Type of project plugin
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PluginType {
    /// Plugin name
    pub name: String,
    /// Plugin description
    pub description: String,
    /// Plugin node value type
    pub node_value_type: Option<NodeValueType>,
    /// Plugin arc value type
    pub arc_value_type: Option<ArcValueType>,
}

/// Type of project permission
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionType {
    /// Permission key
    pub key: String,
    /// Permission description
    pub description: String,
}

/// Type of project user
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUserType {
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
    /// User status in project
    pub status: ProjectUserStatusValue,
    /// User permissions in project
    pub permissions: Option<Vec<String>>,
}

#[derive(FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsInType {
    /// Project group filter
    pub group: ProjectGroupFilterType,
    /// User statuses
    pub statuses: Option<Vec<ProjectUserStatusValue>>,
    /// Search string
    pub search: Option<String>,
    /// Is project archived
    #[field(name = "isArchived")]
    pub is_archived: Option<bool>,
    /// Start of created at interval
    #[field(name = "createdAtStart")]
    pub created_at_start: Option<DateTimeWrapper>,
    /// Whether to include start to created at interval
    #[field(name = "createdAtIncludeStart")]
    pub created_at_include_start: Option<bool>,
    /// End of created at interval
    #[field(name = "createdAtEnd")]
    pub created_at_end: Option<DateTimeWrapper>,
    /// Whether to include end to created at interval
    #[field(name = "createdAtIncludeEnd")]
    pub created_at_include_end: Option<bool>,
    /// Start of updated at interval
    #[field(name = "updatedAtStart")]
    pub updated_at_start: Option<DateTimeWrapper>,
    /// Whether to include start to updated at interval
    #[field(name = "updatedAtIncludeStart")]
    pub updated_at_include_start: Option<bool>,
    /// End of updated at interval
    #[field(name = "updatedAtEnd")]
    pub updated_at_end: Option<DateTimeWrapper>,
    /// Whether to include end to updated at interval
    #[field(name = "updatedAtIncludeEnd")]
    pub updated_at_include_end: Option<bool>,
    /// Page number
    pub page: Option<u16>,
    /// Number of records per page
    #[field(name = "perPage")]
    pub per_page: Option<u16>,
}

/// Interval input type
#[derive(Deserialize, FromForm, JsonSchema)]
pub struct IntervalInType<T: JsonSchema> {
    /// Start of interval
    pub start: Option<T>,
    /// Whether to include start
    pub include_start: bool,
    /// End of interval
    pub end: Option<T>,
    /// Whether to include end
    pub include_end: bool,
}

/// Pagination input type
#[derive(Deserialize)]
pub struct PaginationInType {
    /// Page number
    pub page: u16,
    /// Number of records per page
    pub per_page: u16,
}

/// Pagination output type
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationOutType<T> {
    /// Pagination data
    pub data: Vec<T>,
    /// Total count of pages
    pub total_pages: i32,
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
