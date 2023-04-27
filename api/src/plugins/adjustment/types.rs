use super::models::DynamicModelType;
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
    /// Resulting chromosome of adjustment run
    pub result_chromosome: Option<AdjustmentChromosomeGenerationOutType>,
}

/// Type of adjustment generation
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentGenerationOutType {
    /// Adjustment generation identifier
    pub id: i32,
    /// Adjustment generation number
    pub number: i32,
    /// Adjustment generation average fitness
    pub fitness: f64,
    /// Adjustment generation population
    pub population: Vec<AdjustmentChromosomeOutType>,
}

/// Type of adjustment chromosome
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentChromosomeOutType {
    /// Adjustment chromosome identifier
    pub id: i32,
    /// Adjustment chromosome fitness
    pub fitness: f64,
    /// Adjustment chromosome concept values
    pub concept_values: Vec<AdjustmentConceptValueOutType>,
    /// Adjustment chromosome connection values
    pub connection_values: Vec<AdjustmentConnectionValueOutType>,
}

/// Type of adjustment chromosome with generation information
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentChromosomeGenerationOutType {
    /// Adjustment chromosome identifier
    pub id: i32,
    /// Adjustment chromosome fitness
    pub fitness: f64,
    /// Adjustment generation identifier
    pub generation_id: i32,
    /// Adjustment generation number
    pub generation_number: i32,
    /// Adjustment generation average fitness
    pub generation_fitness: f64,
    /// Adjustment chromosome concept values
    pub concept_values: Vec<AdjustmentConceptValueOutType>,
    /// Adjustment chromosome connection values
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
