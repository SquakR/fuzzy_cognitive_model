use crate::models::{ProjectUserStatus, ProjectUserStatusValue, UserPermission};
use crate::response::ServiceResult;
use crate::schema::user_permissions;
use crate::services::project_services;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn can_change_users(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    let last_status =
        project_services::find_last_status_by_project(connection, project_id, user_id)?;
    Ok(has_permission(connection, &last_status, "can_change_users"))
}

fn has_permission(
    connection: &mut PgConnection,
    last_status: &ProjectUserStatus,
    key: &str,
) -> bool {
    match last_status.status {
        ProjectUserStatusValue::Creator => return true,
        ProjectUserStatusValue::Member => {}
        _ => return false,
    }
    if let Err(_) = user_permissions::table
        .filter(user_permissions::permission_key.eq(key))
        .filter(user_permissions::project_user_id.eq(last_status.project_user_id))
        .first::<UserPermission>(connection)
    {
        return false;
    }
    true
}
