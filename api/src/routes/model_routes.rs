use crate::db;
use crate::models::User;
use crate::request::UserLocale;
use crate::response::PathResult;
use crate::services::model_services;
use crate::types::{NodeInCreateType, NodeOutType, UserOutType};
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Get active users
#[openapi(tag = "model")]
#[get("/project/<project_id>/active_users")]
pub async fn get_active_users(
    project_id: i32,
    user: User,
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<Json<Vec<UserOutType>>, UserLocale> {
    let conn = &mut db::establish_connection();
    let users = match project_service
        .get_active_users(conn, &user, project_id)
        .await
    {
        Ok(users) => users
            .into_iter()
            .map(UserOutType::from)
            .collect::<Vec<UserOutType>>(),
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(users)), locale)
}

/// Create new node
#[openapi(tag = "model")]
#[post("/project/<project_id>/node", format = "json", data = "<node_in>")]
pub async fn create_node(
    project_id: i32,
    node_in: Json<NodeInCreateType>,
    user: User,
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<Json<NodeOutType>, UserLocale> {
    let conn = &mut db::establish_connection();
    let node = match model_services::create_node(
        conn,
        project_service,
        &user,
        project_id,
        node_in.into_inner(),
    )
    .await
    {
        Ok(node) => node,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(node)), locale)
}
