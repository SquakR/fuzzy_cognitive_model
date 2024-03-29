use crate::db;
use crate::models::{ProjectUserStatusValue, User};
use crate::plugins::Plugins;
use crate::response::{
    PathEmptyResult, PathResult, ToPathEmptyResult, ToPathResult, ToServiceResult,
};
use crate::services::{
    permission_services, plugin_services, project_services, project_user_services,
};
use crate::types::{
    IntervalInType, PaginationInType, PaginationOutType, PermissionType, PluginType, ProjectInType,
    ProjectOutType, ProjectUserType, ProjectsInType,
};
use crate::web_socket::{WebSocketAdjustmentRunService, WebSocketModelService};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Create new project
#[openapi(tag = "projects")]
#[post("/project", format = "json", data = "<project_in>")]
pub fn create_project(project_in: Json<ProjectInType>, user: User) -> PathResult<ProjectOutType> {
    let conn = &mut db::establish_connection();
    let project = project_services::create_project(conn, &user, project_in.into_inner())?;
    ProjectOutType::from_project(conn, project).to_path_result()
}

/// Get project
#[openapi(tag = "projects")]
#[get("/projects/<project_id>")]
pub fn get_project(project_id: i32, user: User) -> PathResult<ProjectOutType> {
    let conn = &mut db::establish_connection();
    project_services::get_project(conn, &user, project_id).to_path_result()
}

/// Get projects
#[openapi(tag = "projects")]
#[get("/projects?<projects_in..>")]
pub fn get_projects(
    projects_in: ProjectsInType,
    user: User,
) -> PathResult<PaginationOutType<ProjectOutType>> {
    let conn = &mut db::establish_connection();
    let created_at =
        if projects_in.created_at_start.is_some() || projects_in.created_at_end.is_some() {
            Some(IntervalInType {
                start: projects_in.created_at_start.map(|c| c.0),
                include_start: projects_in.created_at_include_start.unwrap_or(true),
                end: projects_in.created_at_end.map(|c| c.0),
                include_end: projects_in.created_at_include_end.unwrap_or(true),
            })
        } else {
            None
        };
    let updated_at =
        if projects_in.updated_at_start.is_some() || projects_in.updated_at_end.is_some() {
            Some(IntervalInType {
                start: projects_in.updated_at_start.map(|c| c.0),
                include_start: projects_in.updated_at_include_start.unwrap_or(true),
                end: projects_in.updated_at_end.map(|c| c.0),
                include_end: projects_in.updated_at_include_end.unwrap_or(true),
            })
        } else {
            None
        };
    let pagination = PaginationInType {
        page: projects_in.page.unwrap_or(1),
        per_page: projects_in.per_page.unwrap_or(10),
    };
    project_services::paginate_projects(
        conn,
        &user,
        projects_in.group,
        projects_in.statuses,
        projects_in.search,
        projects_in.is_archived,
        created_at,
        updated_at,
        pagination,
    )
    .to_path_result()
}

/// Get plugins
#[openapi(tag = "projects")]
#[get("/plugins")]
pub fn get_plugins(_user: User) -> PathResult<Vec<PluginType>> {
    let conn = &mut db::establish_connection();
    let plugins = plugin_services::get_plugins(conn).to_service_result()?;
    let plugin_types = PluginType::from_plugins(conn, plugins)?;
    Ok(Json(plugin_types))
}

/// Get permissions
#[openapi(tag = "projects")]
#[get("/permissions")]
pub fn get_permissions(_user: User) -> PathResult<Vec<PermissionType>> {
    let conn = &mut db::establish_connection();
    let permissions = permission_services::get_permissions(conn)
        .to_service_result()?
        .into_iter()
        .map(PermissionType::from)
        .collect();
    Ok(Json(permissions))
}

/// Get project users
#[openapi(tag = "projects")]
#[get("/projects/<project_id>/users?<statuses>&<search>&<page>&<per_page>")]
pub fn get_project_users(
    project_id: i32,
    statuses: Option<Vec<ProjectUserStatusValue>>,
    search: Option<&str>,
    page: Option<u16>,
    per_page: Option<u16>,
    user: User,
) -> PathResult<PaginationOutType<ProjectUserType>> {
    let conn = &mut db::establish_connection();
    let pagination_in = PaginationInType {
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(15),
    };
    project_user_services::paginate_project_users(
        conn,
        &user,
        project_id,
        statuses,
        search.map(|s| s.to_owned()),
        pagination_in,
    )
    .to_path_result()
}

/// Change project
#[openapi(tag = "projects")]
#[put("/projects/<project_id>", format = "json", data = "<project_in>")]
pub fn change_project(
    project_id: i32,
    project_in: Json<ProjectInType>,
    user: User,
    plugins: &Plugins,
) -> PathResult<ProjectOutType> {
    let conn = &mut db::establish_connection();
    let project = project_services::change_project(
        conn,
        &user,
        project_id,
        plugins,
        project_in.into_inner(),
    )?;
    ProjectOutType::from_project(conn, project).to_path_result()
}

/// Set project plugins
#[openapi(tag = "projects")]
#[post(
    "/projects/<project_id>/plugins",
    format = "json",
    data = "<new_plugins>"
)]
pub fn set_project_plugins(
    project_id: i32,
    new_plugins: Json<Vec<String>>,
    user: User,
    plugins: &Plugins,
) -> PathResult<Vec<String>> {
    let conn = &mut db::establish_connection();
    plugin_services::set_project_plugins(conn, plugins, &user, project_id, new_plugins.into_inner())
        .to_path_result()
}

/// Set project user permissions
#[openapi(tag = "projects")]
#[post(
    "/projects/<project_id>/users/<user_id>/permissions",
    format = "json",
    data = "<new_permissions>"
)]
pub fn set_project_user_permissions(
    project_id: i32,
    user_id: i32,
    new_permissions: Json<Vec<String>>,
    user: User,
) -> PathResult<Vec<String>> {
    let conn = &mut db::establish_connection();
    permission_services::set_project_user_permissions(
        conn,
        &user,
        project_id,
        user_id,
        new_permissions.into_inner(),
    )
    .to_path_result()
}

/// Invite user to project
#[openapi(tag = "projects")]
#[post("/projects/<project_id>/users/<user_id>/invite")]
pub fn invite_user(project_id: i32, user_id: i32, user: User) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    project_user_services::invite_user(conn, &user, project_id, user_id).to_path_empty_result()
}

/// Cancel user invitation to project
#[openapi(tag = "projects")]
#[post("/projects/<project_id>/users/<user_id>/cancel_invitation")]
pub fn cancel_invitation(project_id: i32, user_id: i32, user: User) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    project_user_services::cancel_invitation(conn, &user, project_id, user_id)
        .to_path_empty_result()
}

/// Respond to invitation to project
#[openapi(tag = "projects")]
#[post("/projects/<project_id>/respond_to_invitation?<join>")]
pub fn respond_to_invitation(project_id: i32, join: bool, user: User) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    project_user_services::respond_to_invitation(conn, &user, project_id, join)
        .to_path_empty_result()
}

/// Leave project
#[openapi(tag = "projects")]
#[post("/projects/<project_id>/leave")]
pub fn leave_project(project_id: i32, user: User) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    project_user_services::leave_project(conn, &user, project_id).to_path_empty_result()
}

/// Exclude user from project
#[openapi(tag = "projects")]
#[post("/projects/<project_id>/users/<user_id>/exclude")]
pub fn exclude_user(project_id: i32, user_id: i32, user: User) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    project_user_services::exclude_user(conn, &user, project_id, user_id).to_path_empty_result()
}

/// Delete project
#[openapi(tag = "projects")]
#[delete("/projects/<project_id>")]
pub async fn delete_project(
    project_id: i32,
    user: User,
    model_service: WebSocketModelService,
    adjustment_run_service: WebSocketAdjustmentRunService,
) -> PathEmptyResult {
    let conn = &mut db::establish_connection();
    project_services::delete_project(
        conn,
        model_service,
        adjustment_run_service,
        &user,
        project_id,
    )
    .await?;
    Ok(())
}
