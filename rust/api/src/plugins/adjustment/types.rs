use super::models::DynamicModelType;
use crate::request::DateTimeWrapper;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

/// Type of concept dynamics model
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDynamicModelOutType {
    /// Concept identifier
    pub concept_id: i32,
    /// Concept dynamics model type
    pub dynamic_model_type: Option<DynamicModelType>,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}

/// Stop condition type of genetic algorithm
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StopConditionType {
    /// Maximum number of generations
    pub max_generations: i32,
    /// Maximum number of generations without improvements
    pub max_without_improvements: i32,
    /// Absolute fitness error
    pub error: f64,
}

/// Input data of the genetic algorithm for the structural-parametric adjustment of fuzzy cognitive model
#[derive(Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentInType {
    /// Adjustment run name
    pub name: String,
    /// Adjustment run description
    pub description: String,
    /// Minimum model time
    pub min_model_time: i32,
    /// Maximum model time
    pub max_model_time: i32,
    /// Dynamics mode type
    pub dynamic_model_type: DynamicModelType,
    /// Generation size
    pub generation_size: i32,
    /// Interval of saving generations
    pub generation_save_interval: i32,
    /// Algorithm stop condition
    pub stop_condition: StopConditionType,
}

/// Type of adjustment run
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentRunOutType {
    /// Adjustment run identifier
    pub id: i32,
    /// Model copy identifier
    pub model_copy_id: i32,
    /// Adjustment run name
    pub name: String,
    /// Adjustment run description
    pub description: String,
    /// Minimum model time
    pub min_model_time: i32,
    /// Maximum model time
    pub max_model_time: i32,
    /// Dynamics mode type
    pub dynamic_model_type: DynamicModelType,
    /// Generation size
    pub generation_size: i32,
    /// Interval of saving generations
    pub generation_save_interval: i32,
    /// Algorithm stop condition
    pub stop_condition: StopConditionType,
    /// Adjustment run creation time
    pub created_at: DateTime<Utc>,
    /// Resulting individual of adjustment run
    pub result_individual: Option<AdjustmentIndividualGenerationOutType>,
}

/// Type of adjustment generation
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentGenerationOutType {
    /// Adjustment generation identifier
    pub id: i32,
    /// Adjustment generation number
    pub number: i32,
    /// Adjustment generation average error
    pub error: f64,
}

/// Type of adjustment individual
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentIndividualOutType {
    /// Adjustment individual identifier
    pub id: i32,
    /// Adjustment individual number
    pub number: i32,
    /// Adjustment individual time
    pub time: i32,
    /// Adjustment individual error
    pub error: f64,
    /// Adjustment individual concept values
    pub concept_values: Vec<AdjustmentConceptValueOutType>,
    /// Adjustment individual connection values
    pub connection_values: Vec<AdjustmentConnectionValueOutType>,
}

/// Type of adjustment individual with generation information
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentIndividualGenerationOutType {
    /// Adjustment individual identifier
    pub id: i32,
    /// Adjustment individual number
    pub number: i32,
    /// Adjustment individual time
    pub time: i32,
    /// Adjustment individual error
    pub error: f64,
    /// Adjustment generation identifier
    pub generation_id: i32,
    /// Adjustment generation number
    pub generation_number: i32,
    /// Adjustment generation average error
    pub generation_error: f64,
    /// Adjustment individual concept values
    pub concept_values: Vec<AdjustmentConceptValueOutType>,
    /// Adjustment individual connection values
    pub connection_values: Vec<AdjustmentConnectionValueOutType>,
}

/// Type of adjustment concept value
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentConceptValueOutType {
    /// Adjustment concept value identifier
    pub id: i32,
    /// Concept identifier
    pub concept_id: i32,
    /// Concept value
    pub value: f64,
}

/// Type of adjustment connection value
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentConnectionValueOutType {
    /// Adjustment connection value identifier
    pub id: i32,
    /// Connection identifier
    pub connection_id: i32,
    /// Connection value
    pub value: f64,
}

/// Input type for getting adjustment runs
#[derive(FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentRunsInType {
    // Search string
    pub search: Option<String>,
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
    /// Page number
    pub page: Option<u16>,
    /// Number of records per page
    #[field(name = "perPage")]
    pub per_page: Option<u16>,
}

/// Input type for getting adjustment generations
#[derive(FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentGenerationsInType {
    /// Page number
    pub page: Option<u16>,
    /// Number of records per page
    #[field(name = "perPage")]
    pub per_page: Option<u16>,
}

/// Input type for getting adjustment individuals
#[derive(FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentIndividualInType {
    /// Page number
    pub page: Option<u16>,
    /// Number of records per page
    #[field(name = "perPage")]
    pub per_page: Option<u16>,
}

/// Type of model action
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentRunActionType<T: Clone + Serialize + JsonSchema> {
    /// Project identifier
    pub project_id: i32,
    /// Adjustment run identifier
    pub adjustment_run_id: i32,
    /// Action name
    pub name: String,
    /// Action data
    pub data: T,
}

/// Type of model error
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentRunActionErrorType {
    // Project identifier
    pub project_id: i32,
    /// Adjustment run identifier
    pub adjustment_run_id: i32,
    /// Action error name
    pub name: String,
    /// Action error message
    pub message: String,
}
