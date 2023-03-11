use crate::models::{Project, ProjectUser, ProjectUserStatusValue, User};
use crate::response::AppError;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::project_user_permissions;
use crate::schema::project_user_statuses;
use crate::schema::project_users;
use crate::schema::projects;
use crate::services::permission_services;
use crate::services::project_user_services;
use crate::types::{ProjectInType, ProjectOutType, ProjectUserType, UserOutType};
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_project(
    connection: &mut PgConnection,
    user: &User,
    project_in: ProjectInType,
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

pub fn change_project(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
    project_in: ProjectInType,
) -> ServiceResult<Project> {
    if !permission_services::can_change_project(connection, project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "change_project_forbidden_error",
        )));
    }
    diesel::update(projects::table)
        .filter(projects::id.eq(&project_id))
        .set((
            projects::name.eq(project_in.name),
            projects::description.eq(project_in.description),
            projects::is_public.eq(project_in.is_public),
            projects::is_archived.eq(project_in.is_archived),
        ))
        .get_result::<Project>(connection)
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
    pub fn from_project(connection: &mut PgConnection, project: Project) -> Self {
        ProjectOutType {
            id: project.id,
            name: project.name,
            description: project.description,
            creator: UserOutType::from(project_user_services::find_project_creator(
                connection, project.id,
            )),
            is_public: project.is_public,
            is_archived: project.is_archived,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

impl ProjectUserType {
    pub fn from_users(
        connection: &mut PgConnection,
        current_user: &User,
        project_id: i32,
        users: Vec<User>,
    ) -> ServiceResult<Vec<Self>> {
        let can_change_permissions =
            permission_services::can_change_permissions(connection, project_id, current_user.id)?;
        let mut statuses =
            ProjectUserType::get_project_user_statuses(project_id, &users, connection)?;
        let mut permissions =
            ProjectUserType::get_project_user_permissions(project_id, &users, connection)?;
        let mut result = Vec::new();
        for user in users {
            let status = ProjectUserType::find_last_status(&mut statuses, user.id);
            let permissions = if can_change_permissions || current_user.id == user.id {
                let permissions = match status {
                    ProjectUserStatusValue::Creator => {
                        permission_services::get_permissions(connection)?
                            .into_iter()
                            .map(|permission| permission.key)
                            .collect::<Vec<String>>()
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
        project_id: i32,
        users: &Vec<User>,
        connection: &mut PgConnection,
    ) -> ServiceResult<Vec<(i32, ProjectUserStatusValue, DateTime<Utc>)>> {
        let user_ids = users.iter().map(|u| u.id);
        project_users::table
            .inner_join(project_user_statuses::table)
            .select((
                project_users::user_id,
                project_user_statuses::status,
                project_user_statuses::created_at,
            ))
            .filter(project_users::project_id.eq(project_id))
            .filter(project_users::user_id.eq_any(user_ids))
            .get_results::<(i32, ProjectUserStatusValue, DateTime<Utc>)>(connection)
            .to_service_result()
    }
    fn get_project_user_permissions(
        project_id: i32,
        users: &Vec<User>,
        connection: &mut PgConnection,
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
            .get_results::<(i32, String)>(connection)
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
