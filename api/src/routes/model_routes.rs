use crate::db;
use crate::models::User;
use crate::request::UserLocale;
use crate::response::PathResult;
use crate::services::model_services;
use crate::types::{
    ModelOutType, UserOutType, VertexInChangeDescriptionType, VertexInCreateType, VertexInMoveType,
    VertexOutChangeDescriptionType, VertexOutChangeValueType, VertexOutMoveType, VertexOutType,
};
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Get model
#[openapi(tag = "model")]
#[get("/project/<project_id>")]
pub fn get_model(
    project_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<ModelOutType>, UserLocale> {
    let conn = &mut db::establish_connection();
    let model = match model_services::get_model(conn, &user, project_id) {
        Ok(model) => model,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(model)), locale)
}

/// Get model active users
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

/// Create new vertex
#[openapi(tag = "model")]
#[post("/project/<project_id>/vertex", format = "json", data = "<vertex_in>")]
pub async fn create_vertex(
    project_id: i32,
    vertex_in: Json<VertexInCreateType>,
    user: User,
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<Json<VertexOutType>, UserLocale> {
    let conn = &mut db::establish_connection();
    let vertex = match model_services::create_vertex(
        conn,
        project_service,
        &user,
        project_id,
        vertex_in.into_inner(),
    )
    .await
    {
        Ok(vertex) => vertex,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(vertex)), locale)
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
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<Json<VertexOutChangeDescriptionType>, UserLocale> {
    let conn = &mut db::establish_connection();
    let vertex = match model_services::change_vertex_description(
        conn,
        project_service,
        &user,
        project_id,
        vertex_id,
        vertex_in.into_inner(),
    )
    .await
    {
        Ok(vertex) => vertex,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(vertex)), locale)
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
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<Json<VertexOutChangeValueType>, UserLocale> {
    let conn = &mut db::establish_connection();
    let vertex = match model_services::change_vertex_value(
        conn,
        project_service,
        &user,
        project_id,
        vertex_id,
        value.into_inner(),
    )
    .await
    {
        Ok(vertex) => vertex,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(vertex)), locale)
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
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<Json<VertexOutMoveType>, UserLocale> {
    let conn = &mut db::establish_connection();
    let vertex = match model_services::move_vertex(
        conn,
        project_service,
        &user,
        project_id,
        vertex_id,
        vertex_in.into_inner(),
    )
    .await
    {
        Ok(vertex) => vertex,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(vertex)), locale)
}

/// Delete vertex
#[openapi(tag = "model")]
#[delete("/project/<project_id>/vertex/<vertex_id>")]
pub async fn delete_vertex(
    project_id: i32,
    vertex_id: i32,
    user: User,
    locale: UserLocale,
    project_service: WebSocketProjectService,
) -> PathResult<(), UserLocale> {
    let conn = &mut db::establish_connection();
    if let Err(app_error) =
        model_services::delete_vertex(conn, project_service, &user, project_id, vertex_id).await
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}
