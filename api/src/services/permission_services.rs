use crate::models::{ProjectUserStatusValue, UserPermission};
use crate::response::ServiceResult;
use crate::schema::user_permissions;
use crate::services::project_services;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
    if let Err(_) = user_permissions::table
        .filter(user_permissions::permission_key.eq(key))
        .filter(user_permissions::project_user_id.eq(last_status.project_user_id))
        .first::<UserPermission>(connection)
    {
        return Ok(false);
    }
    Ok(true)
}
