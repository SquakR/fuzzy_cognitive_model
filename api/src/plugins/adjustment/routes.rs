use super::models::DynamicModelType;
use super::services::{adjustment_services, concept_dynamic_model_services};
use super::types::{AdjustmentInType, AdjustmentRunOutType, ConceptDynamicModelOutType};
use crate::db;
use crate::models::User;
use crate::plugins::Plugins;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change concept concept dynamic model type
#[openapi(tag = "adjustment")]
#[patch(
    "/concepts/<concept_id>/change_dynamic_model_type",
    format = "json",
    data = "<dynamic_model_type>"
)]
pub async fn change_dynamic_model_type(
    concept_id: i32,
    dynamic_model_type: Json<Option<DynamicModelType>>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ConceptDynamicModelOutType>> {
    let conn = &mut db::establish_connection();
    concept_dynamic_model_services::change_dynamic_model_type(
        conn,
        project_service,
        &user,
        concept_id,
        dynamic_model_type.into_inner(),
    )
    .await
    .to_path_result()
}

/// Run genetic algorithm for the structural-parametric adjustment of fuzzy cognitive model
#[openapi(tag = "adjustment")]
#[post(
    "/projects/<project_id>/adjust",
    format = "json",
    data = "<adjustment_in>"
)]
pub async fn adjust(
    project_id: i32,
    adjustment_in: Json<AdjustmentInType>,
    user: User,
    plugins: &Plugins,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<AdjustmentRunOutType>> {
    let conn = db::establish_connection();
    adjustment_services::adjust(
        conn,
        plugins,
        project_service,
        &user,
        project_id,
        adjustment_in.into_inner(),
    )
    .await
    .to_path_result()
}
