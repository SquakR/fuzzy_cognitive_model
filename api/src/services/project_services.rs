use crate::models::{Project, ProjectUser, ProjectUserStatus, ProjectUserStatusValue, User};
use crate::response::AppError;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::project_user_statuses;
use crate::schema::project_users;
use crate::schema::projects;
use crate::schema::users;
use crate::services::permission_services;
use crate::types::{
    CancelInvitationType, InvitationResponseType, InvitationType, ProjectInCreateType,
    ProjectOutType, UserOutType,
};
use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_project(
    connection: &mut PgConnection,
    user: &User,
    project_in: ProjectInCreateType,
) -> ServiceResult<Project> {
    let project = diesel::insert_into(projects::table)
        .values((
            projects::name.eq(project_in.name),
            projects::description.eq(project_in.description),
            projects::is_public.eq(project_in.is_public),
            projects::is_archived.eq(project_in.is_archived),
        ))
        .get_result::<Project>(connection)
        .to_service_result()?;
    let project_user = diesel::insert_into(project_users::table)
        .values((
            project_users::project_id.eq(project.id),
            project_users::user_id.eq(user.id),
        ))
        .get_result::<ProjectUser>(connection)
        .to_service_result()?;
    diesel::insert_into(project_user_statuses::table)
        .values((
            project_user_statuses::project_user_id.eq(project_user.id),
            project_user_statuses::status.eq(ProjectUserStatusValue::Creator),
        ))
        .execute(connection)
        .to_service_result()?;
    Ok(project)
}

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
    user_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    project_user_statuses::table
        .inner_join(project_users::table.inner_join(users::table))
        .select(project_user_statuses::all_columns)
        .filter(project_users::id.eq(project_user_id))
        .filter(users::id.eq(user_id))
        .order(project_user_statuses::created_at.desc())
        .first::<ProjectUserStatus>(connection)
        .to_service_result_find(String::from("last_status_not_found_error"))
}

pub fn try_find_last_status_by_project_user(
    connection: &mut PgConnection,
    project_user_id: i32,
    user_id: i32,
) -> ServiceResult<Option<ProjectUserStatus>> {
    project_user_statuses::table
        .inner_join(project_users::table.inner_join(users::table))
        .select(project_user_statuses::all_columns)
        .filter(project_users::id.eq(project_user_id))
        .filter(users::id.eq(user_id))
        .order(project_user_statuses::created_at.desc())
        .first::<ProjectUserStatus>(connection)
        .optional()
        .to_service_result()
}

pub fn invite_user(
    connection: &mut PgConnection,
    user: &User,
    invitation: InvitationType,
) -> ServiceResult<(ProjectUser, ProjectUserStatus)> {
    if !permission_services::can_change_users(connection, invitation.project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "invite_user_forbidden_error",
        )));
    }
    let project_user_result =
        try_find_project_user(connection, invitation.project_id, invitation.user_id)?;
    let project_user = if let Some(project_user) = project_user_result {
        let last_status =
            try_find_last_status_by_project_user(connection, project_user.id, invitation.user_id)?;
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
            return Err(AppError::ForbiddenError(error.to_owned()));
        }
        project_user
    } else {
        diesel::insert_into(project_users::table)
            .values((
                project_users::project_id.eq(invitation.project_id),
                project_users::user_id.eq(invitation.user_id),
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
    cancel_invitation: CancelInvitationType,
) -> ServiceResult<ProjectUserStatus> {
    if !permission_services::can_change_users(connection, cancel_invitation.project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "cancel_invitation_forbidden_error",
        )));
    }
    let project_user = find_project_user(
        connection,
        cancel_invitation.project_id,
        cancel_invitation.user_id,
    )?;
    let last_status =
        find_last_status_by_project_user(connection, project_user.id, cancel_invitation.user_id)?;
    match last_status.status {
        ProjectUserStatusValue::Invited => {}
        _ => {
            return Err(AppError::ForbiddenError(String::from(
                "there_is_no_invitation_error",
            )))
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
    invitation_response: InvitationResponseType,
) -> ServiceResult<ProjectUserStatus> {
    let project_user = find_project_user(connection, invitation_response.project_id, user.id)?;
    let last_status = find_last_status_by_project_user(connection, project_user.id, user.id)?;
    match last_status.status {
        ProjectUserStatusValue::Invited => {}
        _ => {
            return Err(AppError::ForbiddenError(String::from(
                "there_is_no_invitation_error",
            )))
        }
    }
    let status_value = if invitation_response.join {
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
    let last_status = find_last_status_by_project_user(connection, project_user.id, user.id)?;
    match last_status.status {
        ProjectUserStatusValue::Member => {}
        _ => {
            return Err(AppError::ForbiddenError(String::from(
                "leave_project_error",
            )))
        }
    }
    diesel::insert_into(project_user_statuses::table)
        .values((
            project_user_statuses::project_user_id.eq(project_user.id),
            project_user_statuses::status.eq(ProjectUserStatusValue::Left),
        ))
        .get_result::<ProjectUserStatus>(connection)
        .to_service_result()
}

impl From<(Project, &mut PgConnection)> for ProjectOutType {
    fn from((project, connection): (Project, &mut PgConnection)) -> Self {
        ProjectOutType {
            id: project.id,
            name: project.name,
            description: project.description,
            creator: UserOutType::from(find_project_creator(connection, project.id)),
            is_public: project.is_public,
            is_archived: project.is_archived,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
