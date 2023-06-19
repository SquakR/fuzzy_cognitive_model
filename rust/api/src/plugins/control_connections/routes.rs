use super::services;
use super::types::ControlConnectionOutType;
use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketModelService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change whether connection is control
#[openapi(tag = "control connections")]
#[patch(
    "/connections/<connection_id>/change_is_control",
    format = "json",
    data = "<is_control>"
)]
pub async fn change_connection_is_control(
    connection_id: i32,
    is_control: Json<bool>,
    user: User,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ControlConnectionOutType>> {
    let conn = &mut db::establish_connection();
    services::set_is_control(
        conn,
        model_service,
        &user,
        connection_id,
        is_control.into_inner(),
    )
    .await
    .to_path_result()
}
