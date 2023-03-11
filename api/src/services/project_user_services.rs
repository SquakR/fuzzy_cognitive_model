use crate::models::{ProjectUser, ProjectUserStatus, ProjectUserStatusValue, User};
use crate::pagination::Paginate;
use crate::response::AppError;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::project_user_statuses;
use crate::schema::project_users;
use crate::schema::projects;
use crate::schema::users;
use crate::services::permission_services;
use crate::services::project_services;
use crate::services::user_services;
use crate::types::{PaginationInType, PaginationOutType, ProjectUserType};
use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn find_project_creator(connection: &mut PgConnection, project_id: i32) -> User {
    project_user_statuses::table
        .inner_join(project_users::table.inner_join(users::table))
        .select(users::all_columns)
        .filter(project_users::project_id.eq(project_id))
        .filter(project_user_statuses::status.eq(ProjectUserStatusValue::Creator))
        .first::<User>(connection)
        .unwrap()
}

pub fn find_project_user(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<ProjectUser> {
    project_users::table
        .filter(project_users::project_id.eq(project_id))
        .filter(project_users::user_id.eq(user_id))
        .first::<ProjectUser>(connection)
        .to_service_result_find(String::from("project_user_not_found_error"))
}

pub fn try_find_project_user(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<Option<ProjectUser>> {
    project_users::table
        .filter(project_users::project_id.eq(project_id))
        .filter(project_users::user_id.eq(user_id))
        .first::<ProjectUser>(connection)
        .optional()
        .to_service_result()
}

pub fn find_last_status_by_project(
    connection: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    project_user_statuses::table
        .inner_join(
            project_users::table
                .inner_join(projects::table)
                .inner_join(users::table),
        )
        .select(project_user_statuses::all_columns)
        .filter(projects::id.eq(project_id))
        .filter(users::id.eq(user_id))
        .order(project_user_statuses::created_at.desc())
        .first::<ProjectUserStatus>(connection)
        .to_service_result_find(String::from("last_status_not_found_error"))
}

pub fn find_last_status_by_project_user(
    connection: &mut PgConnection,
    project_user_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    project_user_statuses::table
        .inner_join(project_users::table.inner_join(users::table))
        .select(project_user_statuses::all_columns)
        .filter(project_users::id.eq(project_user_id))
        .order(project_user_statuses::created_at.desc())
        .first::<ProjectUserStatus>(connection)
        .to_service_result_find(String::from("last_status_not_found_error"))
}

pub fn try_find_last_status_by_project_user(
    connection: &mut PgConnection,
    project_user_id: i32,
) -> ServiceResult<Option<ProjectUserStatus>> {
    project_user_statuses::table
        .inner_join(project_users::table.inner_join(users::table))
        .select(project_user_statuses::all_columns)
        .filter(project_users::id.eq(project_user_id))
        .order(project_user_statuses::created_at.desc())
        .first::<ProjectUserStatus>(connection)
        .optional()
        .to_service_result()
}

pub fn is_project_member(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
) -> ServiceResult<bool> {
    let last_status = find_last_status_by_project(connection, project_id, user.id)?;
    match last_status.status {
        ProjectUserStatusValue::Creator | ProjectUserStatusValue::Member => Ok(true),
        _ => Ok(false),
    }
}

pub fn paginate_project_users(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
    statuses: Vec<ProjectUserStatusValue>,
    pagination: PaginationInType,
) -> ServiceResult<PaginationOutType<ProjectUserType>> {
    let project = project_services::find_project_by_id(connection, project_id)?;
    if !project.is_public && !is_project_member(connection, user, project_id)? {
        return Err(AppError::ForbiddenError(String::from(
            "view_project_forbidden_error",
        )));
    }
    let statuses = if statuses.len() > 0 {
        statuses
    } else {
        vec![
            ProjectUserStatusValue::Creator,
            ProjectUserStatusValue::Member,
        ]
    };
    let mut can_change_users: Option<bool> = None;
    for status in statuses.iter() {
        match status {
            ProjectUserStatusValue::Creator | ProjectUserStatusValue::Member => {}
            _ => {
                can_change_users = Some(can_change_users.unwrap_or(
                    permission_services::can_change_users(connection, project_id, user.id)?,
                ));
                if !can_change_users.unwrap() {
                    return Err(AppError::ForbiddenError(String::from(
                        "view_project_users_forbidden_error",
                    )));
                }
            }
        }
    }
    let (users, total_pages) = user_services::filter_users(&pagination.search)
        .inner_join(project_users::table.inner_join(projects::table))
        .select(users::all_columns)
        .filter(projects::id.eq(project_id))
        .paginate(pagination.page as i64)
        .per_page(pagination.per_page as i64)
        .load_and_count_pages::<User>(connection)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: ProjectUserType::from_users(connection, user, project.id, users)?
            .into_iter()
            .filter(|u| statuses.contains(&u.status))
            .collect::<Vec<ProjectUserType>>(),
        total_pages: total_pages as i32,
    })
}

pub fn invite_user(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<(ProjectUser, ProjectUserStatus)> {
    if !permission_services::can_change_users(connection, project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "invite_user_forbidden_error",
        )));
    }
    let project_user_result = try_find_project_user(connection, project_id, user_id)?;
    let project_user = if let Some(project_user) = project_user_result {
        let last_status = try_find_last_status_by_project_user(connection, project_user.id)?;
        let error = match last_status {
            Some(last_status) => match last_status.status {
                ProjectUserStatusValue::Creator | ProjectUserStatusValue::Member => {
                    Some("invite_user_member_error")
                }
                ProjectUserStatusValue::Invited => Some("invite_user_invited_error"),
                ProjectUserStatusValue::Rejected => {
                    if Utc::now() - last_status.created_at < Duration::days(1) {
                        Some("invite_user_rejected_error")
                    } else {
                        None
                    }
                }
                _ => None,
            },
            None => None,
        };
        if let Some(error) = error {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!(error, locale = locale)
            })));
        }
        project_user
    } else {
        diesel::insert_into(project_users::table)
            .values((
                project_users::project_id.eq(project_id),
                project_users::user_id.eq(user_id),
            ))
            .get_result::<ProjectUser>(connection)
            .to_service_result()?
    };
    let status = diesel::insert_into(project_user_statuses::table)
        .values((
            project_user_statuses::project_user_id.eq(project_user.id),
            project_user_statuses::status.eq(ProjectUserStatusValue::Invited),
        ))
        .get_result::<ProjectUserStatus>(connection)
        .to_service_result()?;
    Ok((project_user, status))
}

pub fn cancel_invitation(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    if !permission_services::can_change_users(connection, project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "cancel_invitation_forbidden_error",
        )));
    }
    let project_user = find_project_user(connection, project_id, user_id)?;
    let last_status = find_last_status_by_project_user(connection, project_user.id)?;
    match last_status.status {
        ProjectUserStatusValue::Invited => {}
        _ => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("there_is_no_invitation_error", locale = locale)
            })))
        }
    }
    diesel::insert_into(project_user_statuses::table)
        .values((
            project_user_statuses::project_user_id.eq(project_user.id),
            project_user_statuses::status.eq(ProjectUserStatusValue::Cancelled),
        ))
        .get_result::<ProjectUserStatus>(connection)
        .to_service_result()
}

pub fn respond_to_invitation(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
    join: bool,
) -> ServiceResult<ProjectUserStatus> {
    let project_user = find_project_user(connection, project_id, user.id)?;
    let last_status = find_last_status_by_project_user(connection, project_user.id)?;
    match last_status.status {
        ProjectUserStatusValue::Invited => {}
        _ => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("there_is_no_invitation_error", locale = locale)
            })))
        }
    }
    let status_value = if join {
        ProjectUserStatusValue::Member
    } else {
        ProjectUserStatusValue::Rejected
    };
    diesel::insert_into(project_user_statuses::table)
        .values((
            project_user_statuses::project_user_id.eq(project_user.id),
            project_user_statuses::status.eq(status_value),
        ))
        .get_result::<ProjectUserStatus>(connection)
        .to_service_result()
}

pub fn leave_project(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    let project_user = find_project_user(connection, project_id, user.id)?;
    let last_status = find_last_status_by_project_user(connection, project_user.id)?;
    match last_status.status {
        ProjectUserStatusValue::Member => {}
        _ => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("leave_project_error", locale = locale)
            })))
        }
    }
    permission_services::delete_project_user_permissions(connection, project_user.id)?;
    diesel::insert_into(project_user_statuses::table)
        .values((
            project_user_statuses::project_user_id.eq(project_user.id),
            project_user_statuses::status.eq(ProjectUserStatusValue::Left),
        ))
        .get_result::<ProjectUserStatus>(connection)
        .to_service_result()
}
