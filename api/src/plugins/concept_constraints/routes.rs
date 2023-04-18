use super::services;
use super::types::{ConceptConstraintInChangeType, ConceptConstraintOutType};
use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change concept constraint
#[openapi(tag = "concept constraint")]
#[patch(
    "/concepts/<concept_id>/change_concept_constraint",
    format = "json",
    data = "<concept_constraint_in>"
)]
pub async fn change_concept_constraint(
    concept_id: i32,
    concept_constraint_in: Json<ConceptConstraintInChangeType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ConceptConstraintOutType>> {
    let conn = &mut db::establish_connection();
    services::change_concept_constraint(
        conn,
        project_service,
        &user,
        concept_id,
        concept_constraint_in.into_inner(),
    )
    .await
    .to_path_result()
}
