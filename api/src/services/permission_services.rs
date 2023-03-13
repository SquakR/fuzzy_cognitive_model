use crate::models::{Permission, ProjectUserPermission, ProjectUserStatusValue, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{permissions, project_user_permissions};
use crate::services::project_user_services;
use crate::types::PermissionType;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn set_project_user_permissions(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    user_id: i32,
    permissions: Vec<String>,
) -> ServiceResult<Vec<String>> {
    if !can_change_permissions(conn, project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "change_permissions_forbidden_error",
        )));
    }
    let all_permissions = get_permission_keys(conn)?;
    if let Some(index) = permissions
        .iter()
        .position(|permission| !all_permissions.contains(permission))
    {
        return Err(AppError::ValidationError(Box::new(move |locale| {
            t!(
                "invalid_permission_error",
                locale = locale,
                permission = &permissions[index]
            )
        })));
    }
    let project_user = project_user_services::find_project_user(conn, project_id, user_id)
        .to_service_result_find(String::from("project_user_not_found_error"))?;
    let last_status = project_user_services::find_last_status_by_project_user(conn, &project_user)?;
    match last_status.status {
        ProjectUserStatusValue::Member => {}
        ProjectUserStatusValue::Creator => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("change_creator_permissions_error", locale = locale)
            })))
        }
        _ => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("change_not_member_permissions_error", locale = locale)
            })))
        }
    }
    conn.transaction(|conn| {
        delete_project_user_permissions(conn, project_user.id)?;
        let mut insert_rows = vec![];
        for key in permissions {
            insert_rows.push((
                project_user_permissions::project_user_id.eq(project_user.id),
                project_user_permissions::permission_key.eq(key),
            ));
        }
        let permissions = if insert_rows.len() > 0 {
            diesel::insert_into(project_user_permissions::table)
                .values(&insert_rows)
                .get_results::<ProjectUserPermission>(conn)?
        } else {
            vec![]
        };
        Ok(permissions
            .into_iter()
            .map(|permission| permission.permission_key)
            .collect())
    })
    .to_service_result()
}

pub fn delete_project_user_permissions(
    conn: &mut PgConnection,
    project_user_id: i32,
) -> QueryResult<usize> {
    diesel::delete(
        project_user_permissions::table
            .filter(project_user_permissions::project_user_id.eq(project_user_id)),
    )
    .execute(conn)
}

pub fn get_permissions(conn: &mut PgConnection) -> QueryResult<Vec<Permission>> {
    permissions::table.get_results::<Permission>(conn)
}

pub fn get_permission_keys(conn: &mut PgConnection) -> ServiceResult<Vec<String>> {
    Ok(get_permissions(conn)
        .to_service_result()?
        .into_iter()
        .map(|permission| permission.key)
        .collect())
}

pub fn can_change_project(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(conn, project_id, user_id, "can_change_project")
}

pub fn can_change_plugins(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(conn, project_id, user_id, "can_change_plugins")
}

pub fn can_change_users(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(conn, project_id, user_id, "can_change_users")
}

pub fn can_change_permissions(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(conn, project_id, user_id, "can_change_permissions")
}

pub fn can_delete_project(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    has_permission(conn, project_id, user_id, "can_delete_project")
}

fn has_permission(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
    key: &str,
) -> ServiceResult<bool> {
    let last_status = project_user_services::find_last_status_by_project(conn, project_id, user_id)
        .to_service_result_find(String::from("last_status_not_found_error"))?;
    match last_status.status {
        ProjectUserStatusValue::Creator => return Ok(true),
        ProjectUserStatusValue::Member => {}
        _ => return Ok(false),
    }
    if let Err(_) = project_user_permissions::table
        .filter(project_user_permissions::permission_key.eq(key))
        .filter(project_user_permissions::project_user_id.eq(last_status.project_user_id))
        .first::<ProjectUserPermission>(conn)
    {
        return Ok(false);
    }
    Ok(true)
}

impl From<Permission> for PermissionType {
    fn from(permission: Permission) -> Self {
        PermissionType {
            key: permission.key,
            description: permission.description,
        }
    }
}
