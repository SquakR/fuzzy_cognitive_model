use crate::db;
use crate::models::User;
use crate::response::{PathResult, ToPathResult};
use crate::services::model_services;
use crate::types::{
    ArcInCreateType, ArcOutChangeDescriptionType, ArcOutChangeValueType, ArcOutType,
    ModelActionType, ModelOutType, UserOutType, VertexInChangeDescriptionType, VertexInCreateType,
    VertexInMoveType, VertexOutChangeDescriptionType, VertexOutChangeValueType, VertexOutMoveType,
    VertexOutType,
};
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Get model
#[openapi(tag = "model")]
#[get("/project/<project_id>")]
pub fn get_model(project_id: i32, user: User) -> PathResult<ModelOutType> {
    let conn = &mut db::establish_connection();
    model_services::get_model(conn, &user, project_id).to_path_result()
}

/// Get model active users
#[openapi(tag = "model")]
#[get("/project/<project_id>/active_users")]
pub async fn get_active_users(
    project_id: i32,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<Vec<UserOutType>> {
    let conn = &mut db::establish_connection();
    let users = project_service
        .get_active_users(conn, &user, project_id)
        .await?
        .into_iter()
        .map(UserOutType::from)
        .collect::<Vec<UserOutType>>();
    Ok(Json(users))
}

/// Create new vertex
#[openapi(tag = "model")]
#[post("/project/<project_id>/vertex", format = "json", data = "<vertex_in>")]
pub async fn create_vertex(
    project_id: i32,
    vertex_in: Json<VertexInCreateType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<VertexOutType>> {
    let conn = &mut db::establish_connection();
    model_services::create_vertex(
        conn,
        project_service,
        &user,
        project_id,
        vertex_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Change vertex description
#[openapi(tag = "model")]
#[patch(
    "/project/<project_id>/vertex/<vertex_id>/change_description",
    format = "json",
    data = "<vertex_in>"
)]
pub async fn change_vertex_description(
    project_id: i32,
    vertex_id: i32,
    vertex_in: Json<VertexInChangeDescriptionType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<VertexOutChangeDescriptionType>> {
    let conn = &mut db::establish_connection();
    model_services::change_vertex_description(
        conn,
        project_service,
        &user,
        project_id,
        vertex_id,
        vertex_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Change vertex value
#[openapi(tag = "model")]
#[patch(
    "/project/<project_id>/vertex/<vertex_id>/change_value",
    format = "json",
    data = "<value>"
)]
pub async fn change_vertex_value(
    project_id: i32,
    vertex_id: i32,
    value: Json<Option<f64>>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<VertexOutChangeValueType>> {
    let conn = &mut db::establish_connection();
    model_services::change_vertex_value(
        conn,
        project_service,
        &user,
        project_id,
        vertex_id,
        value.into_inner(),
    )
    .await
    .to_path_result()
}

/// Move vertex
#[openapi(tag = "model")]
#[patch(
    "/project/<project_id>/vertex/<vertex_id>/move_vertex",
    format = "json",
    data = "<vertex_in>"
)]
pub async fn move_vertex(
    project_id: i32,
    vertex_id: i32,
    vertex_in: Json<VertexInMoveType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<VertexOutMoveType>> {
    let conn = &mut db::establish_connection();
    model_services::move_vertex(
        conn,
        project_service,
        &user,
        project_id,
        vertex_id,
        vertex_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Delete vertex
#[openapi(tag = "model")]
#[delete("/project/<project_id>/vertex/<vertex_id>")]
pub async fn delete_vertex(
    project_id: i32,
    vertex_id: i32,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<()>> {
    let conn = &mut db::establish_connection();
    model_services::delete_vertex(conn, project_service, &user, project_id, vertex_id)
        .await
        .to_path_result()
}

/// Create new arc
#[openapi(tag = "model")]
#[post("/project/<project_id>/arc", format = "json", data = "<arc_in>")]
pub async fn create_arc(
    project_id: i32,
    arc_in: Json<ArcInCreateType>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ArcOutType>> {
    let conn = &mut db::establish_connection();
    model_services::create_arc(
        conn,
        project_service,
        &user,
        project_id,
        arc_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Change arc description
#[openapi(tag = "model")]
#[patch(
    "/project/<project_id>/arc/<arc_id>/change_description",
    format = "json",
    data = "<description>"
)]
pub async fn change_arc_description(
    project_id: i32,
    arc_id: i32,
    description: Json<String>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ArcOutChangeDescriptionType>> {
    let conn = &mut db::establish_connection();
    model_services::change_arc_description(
        conn,
        project_service,
        &user,
        project_id,
        arc_id,
        description.into_inner(),
    )
    .await
    .to_path_result()
}

/// Change arc value
#[openapi(tag = "model")]
#[patch(
    "/project/<project_id>/arc/<arc_id>/change_value",
    format = "json",
    data = "<value>"
)]
pub async fn change_arc_value(
    project_id: i32,
    arc_id: i32,
    value: Json<f64>,
    user: User,
    project_service: WebSocketProjectService,
) -> PathResult<ModelActionType<ArcOutChangeValueType>> {
    let conn = &mut db::establish_connection();
    model_services::change_arc_value(
        conn,
        project_service,
        &user,
        project_id,
        arc_id,
        value.into_inner(),
    )
    .await
    .to_path_result()
}
