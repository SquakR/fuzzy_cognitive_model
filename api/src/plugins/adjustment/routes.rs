use super::models::DynamicModelType;
use super::services;
use super::types::ConceptDynamicModelOutType;
use crate::db;
use crate::models::User;
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
    services::change_dynamic_model_type(
        conn,
        project_service,
        &user,
        concept_id,
        dynamic_model_type.into_inner(),
    )
    .await
    .to_path_result()
}
