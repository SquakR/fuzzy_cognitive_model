use crate::models::{Project, User};
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::projects;
use crate::schema::users;
use crate::types::{ProjectInCreateType, ProjectOutType, UserOutType};
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
