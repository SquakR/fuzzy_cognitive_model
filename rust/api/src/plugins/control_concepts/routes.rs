use super::services;
use super::types::ControlConceptOutType;
use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change whether concept is control
#[openapi(tag = "control concepts")]
#[patch(
    "/concepts/<concept_id>/change_is_control",
    format = "json",
    data = "<is_control>"
)]
pub async fn change_concept_is_control(
    concept_id: i32,
    is_control: Json<bool>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ControlConceptOutType>> {
    let conn = &mut db::establish_connection();
    services::set_is_control(
        conn,
        project_service,
        &user,
        concept_id,
        is_control.into_inner(),
    )
    .await
    .to_path_result()
}
