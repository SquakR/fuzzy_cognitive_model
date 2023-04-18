use super::services;
use super::types::{TargetConceptInChangeType, TargetConceptOutType};
use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change target concept
#[openapi(tag = "target concepts")]
#[patch(
    "/concepts/<concept_id>/change_target_concept",
    format = "json",
    data = "<target_concept_in>"
)]
pub async fn change_target_concept(
    concept_id: i32,
    target_concept_in: Json<TargetConceptInChangeType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<TargetConceptOutType>> {
    let conn = &mut db::establish_connection();
    services::change_target_concept(
        conn,
        project_service,
        &user,
        concept_id,
        target_concept_in.into_inner(),
    )
    .await
    .to_path_result()
}
