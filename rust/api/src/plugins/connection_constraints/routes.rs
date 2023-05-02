use super::services;
use super::types::{ConnectionConstraintInChangeType, ConnectionConstraintOutType};
use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change connection constraint
#[openapi(tag = "connection constraint")]
#[patch(
    "/connections/<connection_id>/change_connection_constraint",
    format = "json",
    data = "<connection_constraint_in>"
)]
pub async fn change_connection_constraint(
    connection_id: i32,
    connection_constraint_in: Json<ConnectionConstraintInChangeType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ConnectionConstraintOutType>> {
    let conn = &mut db::establish_connection();
    services::change_connection_constraint(
        conn,
        project_service,
        &user,
        connection_id,
        connection_constraint_in.into_inner(),
    )
    .await
    .to_path_result()
}
