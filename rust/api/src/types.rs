use crate::models::{ConceptValueType, ConnectionValueType, ProjectUserStatusValue};
use crate::request::DateTimeWrapper;
use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Metadata, ObjectValidation, Schema, SchemaObject};
use schemars::{Map, Set};
use serde_json::Value;

/// Type of user (expert or researcher)
#[derive(Serialize, Deserialize, JsonSchema)]
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
    /// User preferred locale
    pub locale: Option<String>,
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
    /// User preferred locale
    pub locale: Option<String>,
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
#[derive(Serialize, Deserialize, JsonSchema)]
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
    /// Project concept value type
    pub concept_value_type: ConceptValueType,
    /// Project connection value type
    pub connection_value_type: ConnectionValueType,
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
    /// Project concept value type
    pub concept_value_type: ConceptValueType,
    /// Project connection value type
    pub connection_value_type: ConnectionValueType,
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
    /// Plugin concept value type
    pub concept_value_type: Option<ConceptValueType>,
    /// Plugin connection value type
    pub connection_value_type: Option<ConnectionValueType>,
    /// Plugin dependencies
    pub dependencies: Vec<String>,
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
    /// User preferred locale
    pub locale: Option<String>,
    /// User creation time
    pub created_at: DateTime<Utc>,
    /// User update time
    pub updated_at: DateTime<Utc>,
    /// User status in project
    pub status: ProjectUserStatusValue,
    /// User permissions in project
    pub permissions: Option<Vec<String>>,
}

/// Input type for getting projects
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

/// Type of model action
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelActionType<T: Clone + Serialize + JsonSchema> {
    /// Project identifier
    pub project_id: i32,
    /// Project update time
    pub project_updated_at: DateTime<Utc>,
    /// Action name
    pub name: String,
    /// Action data
    pub data: T,
}

/// Type of model error
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelActionErrorType {
    // Project identifier
    pub project_id: i32,
    /// Action error name
    pub name: String,
    /// Action error message
    pub message: String,
}

/// Type of concept
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptOutType {
    /// Concept identifier
    pub id: i32,
    /// Concept name
    pub name: String,
    /// Concept description
    pub description: String,
    /// Concept value
    pub value: Option<f64>,
    /// Concept project identifier
    pub project_id: i32,
    /// Concept position in x coordinate
    pub x_position: f64,
    /// Concept position in y coordinate
    pub y_position: f64,
    /// Concept plugins data
    pub plugins_data: Value,
    /// Concept creation time
    pub created_at: DateTime<Utc>,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}

/// Type of concept to create
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptInCreateType {
    /// Concept name
    pub name: String,
    /// Concept description
    pub description: String,
    /// Concept value
    pub value: Option<f64>,
    /// Concept position in x coordinate
    pub x_position: f64,
    /// Concept position in y coordinate
    pub y_position: f64,
}

/// Type of result of changing concept description
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptOutChangeDescriptionType {
    /// Concept identifier
    pub id: i32,
    /// Concept name
    pub name: String,
    /// Concept description
    pub description: String,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}

/// Type of concept to change
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptInChangeDescriptionType {
    /// Concept name
    pub name: String,
    /// Concept description
    pub description: String,
}

/// Type of result of changing concept value
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptOutChangeValueType {
    /// Concept identifier
    pub id: i32,
    /// Concept value
    pub value: Option<f64>,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}

/// Type of result of moving concept
#[derive(Clone, Serialize, JsonSchema)]
pub struct ConceptOutMoveType {
    /// Concept identifier
    pub id: i32,
    /// Concept position in x coordinate
    pub x_position: f64,
    /// Concept position in y coordinate
    pub y_position: f64,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}

/// Type of concept to mode
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptInMoveType {
    /// Concept position in x coordinate
    pub x_position: f64,
    /// Concept position in y coordinate
    pub y_position: f64,
}

/// Type of result of deleting concept
#[derive(Clone, Serialize, JsonSchema)]
pub struct ConceptOutDeleteType {
    /// Concept identifier
    pub id: i32,
    /// Project update time
    pub updated_at: DateTime<Utc>,
}

/// Type of connection
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionOutType {
    /// Connection identifier
    pub id: i32,
    /// Connection description
    pub description: String,
    /// Connection value
    pub value: f64,
    /// Connection source concept identifier
    pub source_id: i32,
    /// Connection target concept identifier
    pub target_id: i32,
    /// Connection project identifier
    pub project_id: i32,
    /// Concept plugins data
    pub plugins_data: Value,
    /// Connection creation time
    pub created_at: DateTime<Utc>,
    /// Connection update time
    pub updated_at: DateTime<Utc>,
}

/// Type of connection to create
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionInCreateType {
    /// Connection description
    pub description: String,
    /// Connection value
    pub value: f64,
    /// Connection source concept identifier
    pub source_id: i32,
    /// Connection target concept identifier
    pub target_id: i32,
}

/// Type of result of changing connection description
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionOutChangeDescriptionType {
    /// Connection identifier
    pub id: i32,
    /// Connection description
    pub description: String,
    /// Connection update time
    pub updated_at: DateTime<Utc>,
}

/// Type of result of changing connection value
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionOutChangeValueType {
    /// Connection identifier
    pub id: i32,
    /// Connection value
    pub value: f64,
    /// Connection update time
    pub updated_at: DateTime<Utc>,
}

/// Type of result of deleting connection
#[derive(Clone, Serialize, JsonSchema)]
pub struct ConnectionOutDeleteType {
    /// Connection identifier
    pub id: i32,
    /// Project update time
    pub updated_at: DateTime<Utc>,
}

/// Type of model
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelOutType {
    /// Model project
    pub project: ProjectOutType,
    /// Model concepts
    pub concepts: Vec<ConceptOutType>,
    /// Model connections
    pub connections: Vec<ConnectionOutType>,
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
                    "locale",
                    InstanceType::String,
                    "User preferred locale",
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
