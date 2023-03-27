use crate::models::{ProjectUser, ProjectUserStatus, ProjectUserStatusValue, User};
use crate::pagination::Paginate;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{
    project_user_permissions, project_user_statuses, project_users, projects, users,
};
use crate::services::{permission_services, project_services, user_services};
use crate::types::{PaginationInType, PaginationOutType, ProjectUserType};
use crate::{forbidden_error, validation_error};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

pub fn add_project_user_status(
    conn: &mut PgConnection,
    project_user_id: i32,
    status: ProjectUserStatusValue,
) -> QueryResult<ProjectUserStatus> {
    conn.transaction(|conn| {
        let last_status = diesel::insert_into(project_user_statuses::table)
            .values((
                project_user_statuses::project_user_id.eq(project_user_id),
                project_user_statuses::status.eq(status),
            ))
            .get_result::<ProjectUserStatus>(conn)?;
        diesel::update(project_users::table)
            .filter(project_users::id.eq(&project_user_id))
            .set(project_users::last_status_id.eq(last_status.id))
            .execute(conn)?;
        Ok(last_status)
    })
}

pub fn find_project_creator(conn: &mut PgConnection, project_id: i32) -> QueryResult<User> {
    project_user_statuses::table
        .inner_join(
            project_users::table
                .inner_join(users::table)
                .on(project_user_statuses::project_user_id.eq(project_users::id)),
        )
        .select(users::all_columns)
        .filter(project_users::project_id.eq(project_id))
        .filter(project_user_statuses::status.eq(ProjectUserStatusValue::Creator))
        .first::<User>(conn)
}

pub fn find_project_user(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> QueryResult<ProjectUser> {
    project_users::table
        .filter(project_users::project_id.eq(project_id))
        .filter(project_users::user_id.eq(user_id))
        .first::<ProjectUser>(conn)
}

pub fn find_last_status_by_project(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> QueryResult<ProjectUserStatus> {
    project_user_statuses::table
        .inner_join(
            project_users::table
                .inner_join(projects::table)
                .inner_join(users::table)
                .on(project_user_statuses::project_user_id.eq(project_users::id)),
        )
        .select(project_user_statuses::all_columns)
        .filter(projects::id.eq(project_id))
        .filter(users::id.eq(user_id))
        .order(project_user_statuses::created_at.desc())
        .first::<ProjectUserStatus>(conn)
}

pub fn find_last_status_by_project_user(
    conn: &mut PgConnection,
    project_user: &ProjectUser,
) -> QueryResult<ProjectUserStatus> {
    project_user_statuses::table
        .filter(project_user_statuses::id.eq(project_user.last_status_id))
        .first::<ProjectUserStatus>(conn)
}

pub fn is_project_member(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
) -> ServiceResult<bool> {
    let last_status = find_last_status_by_project(conn, project_id, user.id)
        .to_service_result_find(String::from("last_status_not_found_error"))?;
    match last_status.status {
        ProjectUserStatusValue::Creator | ProjectUserStatusValue::Member => Ok(true),
        _ => Ok(false),
    }
}

pub fn paginate_project_users(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    statuses: Option<Vec<ProjectUserStatusValue>>,
    search: Option<String>,
    pagination: PaginationInType,
) -> ServiceResult<PaginationOutType<ProjectUserType>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let statuses = statuses.unwrap_or(vec![
        ProjectUserStatusValue::Creator,
        ProjectUserStatusValue::Member,
    ]);
    let mut can_change_users: Option<bool> = None;
    for status in statuses.iter() {
        match status {
            ProjectUserStatusValue::Creator | ProjectUserStatusValue::Member => {}
            _ => {
                can_change_users = Some(can_change_users.unwrap_or(
                    permission_services::can_change_users_base(conn, project_id, user.id)?,
                ));
                if !can_change_users.unwrap() {
                    return forbidden_error!("view_project_users_forbidden_error");
                }
            }
        }
    }
    let (users, total_pages) = user_services::filter_users(search)
        .inner_join(
            project_users::table
                .inner_join(
                    project_user_statuses::table
                        .on(project_users::last_status_id.eq(project_user_statuses::id)),
                )
                .inner_join(projects::table),
        )
        .select(users::all_columns)
        .filter(projects::id.eq(project_id))
        .filter(project_user_statuses::status.eq_any(statuses))
        .paginate(pagination.page as i64)
        .per_page(pagination.per_page as i64)
        .load_and_count_pages::<User>(conn)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: ProjectUserType::from_users(conn, user, project.id, users)?,
        total_pages: total_pages as i32,
    })
}

pub fn invite_user(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<(ProjectUser, ProjectUserStatus)> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_users(conn, &project, user.id)?;
    let project_user_result = find_project_user(conn, project_id, user_id)
        .optional()
        .to_service_result()?;
    if let Some(project_user) = &project_user_result {
        let last_status =
            find_last_status_by_project_user(conn, &project_user).to_service_result()?;
        let error = match last_status.status {
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
        };
        if let Some(error) = error {
            return validation_error!(error);
        }
    }
    conn.transaction(|conn| {
        let project_user = if let Some(project_user) = project_user_result {
            project_user
        } else {
            diesel::insert_into(project_users::table)
                .values((
                    project_users::project_id.eq(project_id),
                    project_users::user_id.eq(user_id),
                    project_users::last_status_id.eq(0),
                ))
                .get_result::<ProjectUser>(conn)?
        };
        let status =
            add_project_user_status(conn, project_user.id, ProjectUserStatusValue::Invited)?;
        Ok((project_user, status))
    })
    .to_service_result()
}

pub fn cancel_invitation(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_users(conn, &project, user.id)?;
    let project_user = find_project_user(conn, project_id, user_id)
        .to_service_result_find(String::from("project_user_not_found_error"))?;
    let last_status = find_last_status_by_project_user(conn, &project_user).to_service_result()?;
    match last_status.status {
        ProjectUserStatusValue::Invited => {}
        _ => return validation_error!("there_is_no_invitation_error"),
    }
    add_project_user_status(conn, project_user.id, ProjectUserStatusValue::Cancelled)
        .to_service_result()
}

pub fn respond_to_invitation(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    join: bool,
) -> ServiceResult<ProjectUserStatus> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    project_services::is_not_archived(&project)?;
    let project_user = find_project_user(conn, project_id, user.id)
        .to_service_result_find(String::from("project_user_not_found_error"))?;
    let last_status = find_last_status_by_project_user(conn, &project_user).to_service_result()?;
    match last_status.status {
        ProjectUserStatusValue::Invited => {}
        _ => return validation_error!("there_is_no_invitation_error"),
    }
    let status_value = if join {
        ProjectUserStatusValue::Member
    } else {
        ProjectUserStatusValue::Rejected
    };
    add_project_user_status(conn, project_user.id, status_value).to_service_result()
}

pub fn leave_project(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    project_services::is_not_archived(&project)?;
    let project_user = find_project_user(conn, project_id, user.id)
        .to_service_result_find(String::from("project_user_not_found_error"))?;
    let last_status = find_last_status_by_project_user(conn, &project_user).to_service_result()?;
    match last_status.status {
        ProjectUserStatusValue::Member => {}
        _ => return validation_error!("leave_project_error"),
    }
    conn.transaction(|conn| {
        permission_services::delete_project_user_permissions(conn, project_user.id)?;
        add_project_user_status(conn, project_user.id, ProjectUserStatusValue::Left)
    })
    .to_service_result()
}

pub fn exclude_user(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<ProjectUserStatus> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_users(conn, &project, user.id)?;
    if user.id == user_id {
        return validation_error!("exclude_user_self_error");
    }
    let project_user = find_project_user(conn, project_id, user_id)
        .to_service_result_find(String::from("project_user_not_found_error"))?;
    let last_status = find_last_status_by_project_user(conn, &project_user).to_service_result()?;
    match last_status.status {
        ProjectUserStatusValue::Member => {}
        ProjectUserStatusValue::Creator => return validation_error!("exclude_creator_error"),
        _ => return validation_error!("exclude_not_member_error"),
    }
    conn.transaction(|conn| {
        permission_services::delete_project_user_permissions(conn, project_user.id)?;
        add_project_user_status(conn, project_user.id, ProjectUserStatusValue::Excluded)
    })
    .to_service_result()
}

impl ProjectUserType {
    pub fn from_users(
        conn: &mut PgConnection,
        current_user: &User,
        project_id: i32,
        users: Vec<User>,
    ) -> ServiceResult<Vec<Self>> {
        let can_change_permissions =
            permission_services::can_change_permissions_base(conn, project_id, current_user.id)?;
        let mut statuses = ProjectUserType::get_project_user_statuses(conn, project_id, &users)?;
        let mut permissions =
            ProjectUserType::get_project_user_permissions(conn, project_id, &users)?;
        let mut result = Vec::new();
        for user in users {
            let status = ProjectUserType::find_last_status(&mut statuses, user.id);
            let permissions = if can_change_permissions || current_user.id == user.id {
                let permissions = match status {
                    ProjectUserStatusValue::Creator => {
                        permission_services::get_permission_keys(conn)?
                    }
                    ProjectUserStatusValue::Member => {
                        ProjectUserType::find_user_permissions(&mut permissions, user.id)
                    }
                    _ => vec![],
                };
                Some(permissions)
            } else {
                None
            };
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
                status,
                permissions,
            });
        }
        Ok(result)
    }
    fn get_project_user_statuses(
        conn: &mut PgConnection,
        project_id: i32,
        users: &[User],
    ) -> ServiceResult<Vec<(i32, ProjectUserStatusValue, DateTime<Utc>)>> {
        let user_ids = users.iter().map(|u| u.id);
        project_users::table
            .inner_join(
                project_user_statuses::table
                    .on(project_users::last_status_id.eq(project_user_statuses::id)),
            )
            .select((
                project_users::user_id,
                project_user_statuses::status,
                project_user_statuses::created_at,
            ))
            .filter(project_users::project_id.eq(project_id))
            .filter(project_users::user_id.eq_any(user_ids))
            .get_results::<(i32, ProjectUserStatusValue, DateTime<Utc>)>(conn)
            .to_service_result()
    }
    fn get_project_user_permissions(
        conn: &mut PgConnection,
        project_id: i32,
        users: &[User],
    ) -> ServiceResult<Vec<(i32, String)>> {
        let user_ids = users.iter().map(|u| u.id);
        project_users::table
            .inner_join(project_user_permissions::table)
            .select((
                project_users::user_id,
                project_user_permissions::permission_key,
            ))
            .filter(project_users::project_id.eq(project_id))
            .filter(project_users::user_id.eq_any(user_ids))
            .get_results::<(i32, String)>(conn)
            .to_service_result()
    }
    fn find_last_status(
        statuses: &mut Vec<(i32, ProjectUserStatusValue, DateTime<Utc>)>,
        user_id: i32,
    ) -> ProjectUserStatusValue {
        let status_index = statuses
            .iter()
            .enumerate()
            .filter(|(_, (id, _, _))| *id == user_id)
            .max_by_key(|(_, (_, _, created_at))| *created_at)
            .unwrap()
            .0;
        let (_, status, _) = statuses.remove(status_index);
        status
    }
    fn find_user_permissions(permissions: &mut Vec<(i32, String)>, user_id: i32) -> Vec<String> {
        let permission_indices = permissions
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| *id == user_id)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let mut result = vec![];
        for index in permission_indices.into_iter().rev() {
            result.push(permissions.remove(index).1)
        }
        result.into_iter().rev().collect()
    }
}
