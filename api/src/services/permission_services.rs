use crate::models::{Permission, ProjectUserPermission, ProjectUserStatusValue};
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::permissions;
use crate::schema::project_user_permissions;
use crate::services::project_services;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get_permissions(connection: &mut PgConnection) -> ServiceResult<Vec<Permission>> {
    permissions::table
        .get_results::<Permission>(connection)
        .to_service_result()
}

pub fn can_change_project(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(connection, project_id, user_id, "can_change_project")
}

pub fn can_change_users(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(connection, project_id, user_id, "can_change_users")
}

pub fn can_change_permissions(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(connection, project_id, user_id, "can_change_permissions")
}

pub fn can_delete_project(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(connection, project_id, user_id, "can_delete_project")
}

fn has_permission(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
    key: &str,
) -> ServiceResult<bool> {
    let last_status =
        project_services::find_last_status_by_project(connection, project_id, user_id)?;
    match last_status.status {
        ProjectUserStatusValue::Creator => return Ok(true),
        ProjectUserStatusValue::Member => {}
        _ => return Ok(false),
    }
    if let Err(_) = project_user_permissions::table
        .filter(project_user_permissions::permission_key.eq(key))
        .filter(project_user_permissions::project_user_id.eq(last_status.project_user_id))
        .first::<ProjectUserPermission>(connection)
    {
        return Ok(false);
    }
    Ok(true)
}
