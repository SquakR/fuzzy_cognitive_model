use super::services;
use super::types::ControlVertexOutType;
use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change whether vertex is control
#[openapi(tag = "control vertices")]
#[patch(
    "/vertices/<vertex_id>/change_is_control",
    format = "json",
    data = "<is_control>"
)]
pub async fn change_vertex_is_control(
    vertex_id: i32,
    is_control: Json<bool>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ControlVertexOutType>> {
    let conn = &mut db::establish_connection();
    services::set_is_control(
        conn,
        project_service,
        &user,
        vertex_id,
        is_control.into_inner(),
    )
    .await
    .to_path_result()
}
