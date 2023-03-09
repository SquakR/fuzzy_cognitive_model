use crate::models::{Project, ProjectUser, ProjectUserStatus, ProjectUserStatusValue, User};
use crate::pagination::Paginate;
use crate::response::AppError;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::project_user_statuses;
use crate::schema::project_users;
use crate::schema::projects;
use crate::schema::users;
use crate::services::permission_services;
use crate::services::user_services;
use crate::types::{
    CancelInvitationType, InvitationResponseType, InvitationType, PaginationInType,
    PaginationOutType, ProjectInChangeType, ProjectInCreateType, ProjectOutType, ProjectUserType,
    UserOutType,
};
use chrono::{DateTime, Duration, Utc};
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

pub fn find_project_by_id(
    connection: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Project> {
    projects::table
        .find(project_id)
        .first::<Project>(connection)
        .to_service_result_find(String::from("project_not_found_error"))
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
    let project = find_project_by_id(connection, project_id)?;
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
    let mut query = user_services::filter_users(&pagination.search)
        .inner_join(
            project_users::table
                .inner_join(projects::table)
                .inner_join(project_user_statuses::table),
        )
        .select(users::all_columns)
        .filter(projects::id.eq(project_id));
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
    if statuses.len() > 0 {
        query = query.filter(project_user_statuses::status.eq_any(statuses));
    }
    let (users, total_pages) = query
        .paginate(pagination.page as i64)
        .per_page(pagination.per_page as i64)
        .load_and_count_pages::<User>(connection)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: ProjectUserType::from_users(project.id, users, connection),
        total_pages: total_pages as i32,
    })
}

pub fn change_project(
    connection: &mut PgConnection,
    user: &User,
    project_in: ProjectInChangeType,
) -> ServiceResult<Project> {
    if !permission_services::can_change_project(connection, project_in.project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "change_project_forbidden_error",
        )));
    }
    diesel::update(projects::table)
        .filter(projects::id.eq(&project_in.project_id))
        .set((
            projects::name.eq(project_in.name),
            projects::description.eq(project_in.description),
            projects::is_public.eq(project_in.is_public),
            projects::is_archived.eq(project_in.is_archived),
        ))
        .get_result::<Project>(connection)
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

pub fn delete_project(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
) -> ServiceResult<()> {
    if !permission_services::can_delete_project(connection, project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "delete_project_forbidden_error",
        )));
    }
    diesel::delete(projects::table.filter(projects::id.eq(project_id)))
        .execute(connection)
        .to_service_result()?;
    Ok(())
}

impl ProjectOutType {
    pub fn from_project(project: Project, connection: &mut PgConnection) -> Self {
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

impl ProjectUserType {
    pub fn from_users(
        project_id: i32,
        users: Vec<User>,
        connection: &mut PgConnection,
    ) -> Vec<Self> {
        let user_ids = users.iter().map(|u| u.id);
        let mut statuses = project_users::table
            .inner_join(project_user_statuses::table)
            .select((
                project_users::user_id,
                project_user_statuses::status,
                project_user_statuses::created_at,
            ))
            .filter(project_users::project_id.eq(project_id))
            .filter(project_users::user_id.eq_any(user_ids))
            .get_results::<(i32, ProjectUserStatusValue, DateTime<Utc>)>(connection)
            .unwrap();

        let mut result = Vec::new();
        for user in users {
            let status_index = statuses
                .iter()
                .enumerate()
                .filter(|(_, (user_id, _, _))| *user_id == user.id)
                .max_by_key(|(_, (_, _, created_at))| *created_at)
                .unwrap()
                .0;
            let (_, status, _) = statuses.remove(status_index);
            result.push(ProjectUserType {
                id: user.id,
                username: user.username,
                email: user.email,
                is_email_confirmed: user.is_email_confirmed,
                first_name: user.first_name,
                second_name: user.second_name,
                last_name: user.last_name,
                avatar: user.avatar,
                language: user.language,
                created_at: user.created_at,
                updated_at: user.updated_at,
                status: status,
            });
        }
        result
    }
}
