use crate::db;
use crate::models::{ProjectUserStatusValue, User};
use crate::request::UserLocale;
use crate::response::PathResult;
use crate::services::permission_services;
use crate::services::project_services;
use crate::services::project_user_services;
use crate::types::{
    PaginationInType, PaginationOutType, PermissionType, ProjectInType, ProjectOutType,
    ProjectUserType,
};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Create new project
#[openapi(tag = "projects")]
#[post("/project", format = "json", data = "<project_in>")]
pub fn create_project(
    project_in: Json<ProjectInType>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<ProjectOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let project = match project_services::create_project(connection, &user, project_in.into_inner())
    {
        Ok(project) => project,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(
        Ok(Json(ProjectOutType::from_project(connection, project))),
        locale,
    )
}

/// Get permissions
#[openapi(tag = "projects")]
#[get("/permissions")]
pub fn get_permissions(locale: UserLocale) -> PathResult<Json<Vec<PermissionType>>, UserLocale> {
    let connection = &mut db::establish_connection();
    let permissions = match permission_services::get_permissions(connection) {
        Ok(permissions) => permissions,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(
        Ok(Json(
            permissions
                .into_iter()
                .map(PermissionType::from)
                .collect::<Vec<PermissionType>>(),
        )),
        locale,
    )
}

/// Get project users
#[openapi(tag = "projects")]
#[get("/project/<project_id>/users?<statuses>&<search>&<page>&<per_page>")]
pub fn get_project_users(
    project_id: i32,
    statuses: Option<&str>,
    search: Option<&str>,
    page: Option<u16>,
    per_page: Option<u16>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<PaginationOutType<ProjectUserType>>, UserLocale> {
    let statuses = statuses
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().parse::<ProjectUserStatusValue>())
        .filter_map(Result::ok)
        .collect::<Vec<ProjectUserStatusValue>>();
    let connection = &mut db::establish_connection();
    let pagination_in = PaginationInType {
        search: search.map(|s| s.to_owned()),
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(15),
    };
    let pagination_out = match project_user_services::paginate_project_users(
        connection,
        &user,
        project_id,
        statuses,
        pagination_in,
    ) {
        Ok(pagination_out) => pagination_out,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(pagination_out)), locale)
}

/// Change project
#[openapi(tag = "projects")]
#[put("/project/<project_id>", format = "json", data = "<project_in>")]
pub fn change_project(
    project_id: i32,
    project_in: Json<ProjectInType>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<ProjectOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let project = match project_services::change_project(
        connection,
        &user,
        project_id,
        project_in.into_inner(),
    ) {
        Ok(project) => project,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(
        Ok(Json(ProjectOutType::from_project(connection, project))),
        locale,
    )
}

/// Set project user permissions
#[openapi(tag = "projects")]
#[post(
    "/project/<project_id>/user/<user_id>/permissions",
    format = "json",
    data = "<permissions>"
)]
pub fn set_project_user_permissions(
    project_id: i32,
    user_id: i32,
    permissions: Json<Vec<String>>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<Vec<String>>, UserLocale> {
    let connection = &mut db::establish_connection();
    let permissions = match permission_services::set_project_user_permissions(
        connection,
        &user,
        project_id,
        user_id,
        permissions.into_inner(),
    ) {
        Ok(permissions) => permissions,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(permissions)), locale)
}

/// Invite user to project
#[openapi(tag = "projects")]
#[post("/project/<project_id>/user/<user_id>/invite")]
pub fn invite_user(
    project_id: i32,
    user_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_user_services::invite_user(connection, &user, project_id, user_id)
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Cancel user invitation to project
#[openapi(tag = "projects")]
#[post("/project/<project_id>/user/<user_id>/cancel_invitation")]
pub fn cancel_invitation(
    project_id: i32,
    user_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_user_services::cancel_invitation(connection, &user, project_id, user_id)
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Respond to invitation to project
#[openapi(tag = "projects")]
#[post("/project/<project_id>/respond_to_invitation?<join>")]
pub fn respond_to_invitation(
    project_id: i32,
    join: bool,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_user_services::respond_to_invitation(connection, &user, project_id, join)
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Leave project
#[openapi(tag = "projects")]
#[post("/project/<project_id>/leave")]
pub fn leave_project(
    project_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) = project_user_services::leave_project(connection, &user, project_id) {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Exclude user from project
#[openapi(tag = "projects")]
#[post("/project/<project_id>/user/<user_id>/exclude")]
pub fn exclude_user(
    project_id: i32,
    user_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_user_services::exclude_user(connection, &user, project_id, user_id)
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Delete project
#[openapi(tag = "projects")]
#[delete("/project/<project_id>")]
pub fn delete_project(
    project_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) = project_services::delete_project(connection, &user, project_id) {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}
