use crate::models::{Project, ProjectUser, User, UserPermission};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::project_users;
use crate::schema::projects;
use crate::schema::user_permissions;
use crate::schema::users;
use crate::types::{ProjectInCreateType, ProjectOutType, UserInvitationType, UserOutType};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_project(
    connection: &mut PgConnection,
    user: &User,
    project_in: ProjectInCreateType,
) -> ServiceResult<Project> {
    diesel::insert_into(projects::table)
        .values((
            projects::name.eq(project_in.name),
            projects::description.eq(project_in.description),
            projects::created_by_id.eq(user.id),
            projects::is_public.eq(project_in.is_public),
            projects::is_archived.eq(project_in.is_archived),
        ))
        .get_result::<Project>(connection)
        .to_service_result()
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

pub fn invite_user(
    connection: &mut PgConnection,
    user: &User,
    invitation: UserInvitationType,
) -> ServiceResult<ProjectUser> {
    let project = find_project_by_id(connection, invitation.project_id)?;
    if !has_permission(connection, user, &project, "can_invite_users") {
        return Err(AppError::ForbiddenError(String::from(
            "project_invite_user_forbidden_error",
        )));
    }
    if is_project_member(connection, invitation.user_id, invitation.project_id) {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("project_invite_user_already_exist", locale = locale)
        })));
    }
    diesel::insert_into(project_users::table)
        .values((
            project_users::project_id.eq(project.id),
            project_users::user_id.eq(invitation.user_id),
        ))
        .get_result::<ProjectUser>(connection)
        .to_service_result()
}

fn has_permission(
    connection: &mut PgConnection,
    user: &User,
    project: &Project,
    key: &str,
) -> bool {
    if user.id == project.created_by_id {
        return true;
    }
    let project_user = match project_users::table
        .filter(project_users::project_id.eq(project.id))
        .filter(project_users::user_id.eq(user.id))
        .first::<ProjectUser>(connection)
    {
        Ok(project_user) => project_user,
        Err(_) => return false,
    };
    if !project_user.is_confirmed {
        return false;
    }
    if let Err(_) = user_permissions::table
        .filter(user_permissions::permission_key.eq(key))
        .filter(user_permissions::project_user_id.eq(project_user.id))
        .first::<UserPermission>(connection)
    {
        return false;
    }
    true
}

fn is_project_member(connection: &mut PgConnection, user_id: i32, project_id: i32) -> bool {
    project_users::table
        .filter(project_users::project_id.eq(project_id))
        .filter(project_users::user_id.eq(user_id))
        .first::<ProjectUser>(connection)
        .is_ok()
}

impl From<(Project, &mut PgConnection)> for ProjectOutType {
    fn from((project, connection): (Project, &mut PgConnection)) -> Self {
        ProjectOutType {
            id: project.id,
            name: project.name,
            description: project.description,
            creator: UserOutType::from(
                users::table
                    .find(project.created_by_id)
                    .first::<User>(connection)
                    .unwrap(),
            ),
            is_public: project.is_public,
            is_archived: project.is_archived,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
