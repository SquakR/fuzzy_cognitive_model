use crate::db;
use crate::models::User;
use crate::plugins::Plugins;
use crate::response::{PathResult, ToPathResult};
use crate::services::model_services;
use crate::types::{
    ConceptInMoveType, ConceptInType, ConceptOutChangeType, ConceptOutDeleteType,
    ConceptOutMoveType, ConceptOutType, ConnectionInChangeType, ConnectionInCreateType,
    ConnectionOutChangeType, ConnectionOutDeleteType, ConnectionOutType, ModelActionType,
    ModelOutType, UserOutType,
};
use crate::web_socket::WebSocketModelService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Get model
#[openapi(tag = "model")]
#[get("/projects/<project_id>")]
pub fn get_model(project_id: i32, user: User, plugins: &Plugins) -> PathResult<ModelOutType> {
    let conn = &mut db::establish_connection();
    model_services::get_model(conn, plugins, &user, project_id).to_path_result()
}

/// Get model copy
#[openapi(tag = "model")]
#[get("/models/<model_copy_id>")]
pub fn get_model_copy(model_copy_id: i32, user: User) -> PathResult<ModelOutType> {
    let conn = &mut db::establish_connection();
    model_services::get_model_copy(conn, &user, model_copy_id).to_path_result()
}

/// Get model active users
#[openapi(tag = "model")]
#[get("/projects/<project_id>/active_users")]
pub async fn get_active_users(
    project_id: i32,
    user: User,
    model_service: WebSocketModelService,
) -> PathResult<Vec<UserOutType>> {
    let conn = &mut db::establish_connection();
    let users = model_service
        .get_active_users(conn, &user, project_id)
        .await?
        .into_iter()
        .map(UserOutType::from)
        .collect::<Vec<UserOutType>>();
    Ok(Json(users))
}

/// Create new concept
#[openapi(tag = "model")]
#[post(
    "/projects/<project_id>/concept",
    format = "json",
    data = "<concept_in>"
)]
pub async fn create_concept(
    project_id: i32,
    concept_in: Json<ConceptInType>,
    user: User,
    plugins: &Plugins,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConceptOutType>> {
    let conn = &mut db::establish_connection();
    model_services::create_concept(
        conn,
        plugins,
        model_service,
        &user,
        project_id,
        concept_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Change concept description
#[openapi(tag = "model")]
#[put("/concepts/<concept_id>", format = "json", data = "<concept_in>")]
pub async fn change_concept(
    concept_id: i32,
    concept_in: Json<ConceptInType>,
    user: User,
    plugins: &Plugins,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConceptOutChangeType>> {
    let conn = &mut db::establish_connection();
    model_services::change_concept(
        conn,
        plugins,
        model_service,
        &user,
        concept_id,
        concept_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Move concept
#[openapi(tag = "model")]
#[patch("/concepts/<concept_id>/move", format = "json", data = "<concept_in>")]
pub async fn move_concept(
    concept_id: i32,
    concept_in: Json<ConceptInMoveType>,
    user: User,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConceptOutMoveType>> {
    let conn = &mut db::establish_connection();
    model_services::move_concept(
        conn,
        model_service,
        &user,
        concept_id,
        concept_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Delete concept
#[openapi(tag = "model")]
#[delete("/concepts/<concept_id>")]
pub async fn delete_concept(
    concept_id: i32,
    user: User,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConceptOutDeleteType>> {
    let conn = &mut db::establish_connection();
    model_services::delete_concept(conn, model_service, &user, concept_id)
        .await
        .to_path_result()
}

/// Create new connection
#[openapi(tag = "model")]
#[post(
    "/projects/<project_id>/connection",
    format = "json",
    data = "<connection_in>"
)]
pub async fn create_connection(
    project_id: i32,
    connection_in: Json<ConnectionInCreateType>,
    user: User,
    plugins: &Plugins,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConnectionOutType>> {
    let conn = &mut db::establish_connection();
    model_services::create_connection(
        conn,
        plugins,
        model_service,
        &user,
        project_id,
        connection_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Change connection
#[openapi(tag = "model")]
#[patch(
    "/connections/<connection_id>",
    format = "json",
    data = "<connection_in>"
)]
pub async fn change_connection(
    connection_id: i32,
    connection_in: Json<ConnectionInChangeType>,
    user: User,
    plugins: &Plugins,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConnectionOutChangeType>> {
    let conn = &mut db::establish_connection();
    model_services::change_connection(
        conn,
        plugins,
        model_service,
        &user,
        connection_id,
        connection_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Delete connection
#[openapi(tag = "model")]
#[delete("/connections/<connection_id>")]
pub async fn delete_connection(
    connection_id: i32,
    user: User,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConnectionOutDeleteType>> {
    let conn = &mut db::establish_connection();
    model_services::delete_connection(conn, model_service, &user, connection_id)
        .await
        .to_path_result()
}
