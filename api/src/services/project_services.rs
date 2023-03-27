use crate::models::{Project, ProjectUser, ProjectUserStatusValue, User};
use crate::pagination::Paginate;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{
    plugins, project_plugins, project_user_statuses, project_users, projects, users,
};
use crate::services::{permission_services, plugin_services, project_user_services};
use crate::types::{
    IntervalInType, PaginationInType, PaginationOutType, ProjectGroupFilterType, ProjectInType,
    ProjectOutType, UserOutType,
};
use crate::validation_error;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

pub fn create_project(
    conn: &mut PgConnection,
    user: &User,
    project_in: ProjectInType,
) -> ServiceResult<Project> {
    conn.transaction(|conn| {
        let project = diesel::insert_into(projects::table)
            .values((
                projects::name.eq(project_in.name),
                projects::description.eq(project_in.description),
                projects::is_public.eq(project_in.is_public),
                projects::is_archived.eq(project_in.is_archived),
                projects::vertex_value_type.eq(project_in.vertex_value_type),
                projects::arc_value_type.eq(project_in.arc_value_type),
            ))
            .get_result::<Project>(conn)?;
        let project_user = diesel::insert_into(project_users::table)
            .values((
                project_users::project_id.eq(project.id),
                project_users::user_id.eq(user.id),
                project_users::last_status_id.eq(0),
            ))
            .get_result::<ProjectUser>(conn)?;
        project_user_services::add_project_user_status(
            conn,
            project_user.id,
            ProjectUserStatusValue::Creator,
        )?;
        Ok(project)
    })
    .to_service_result()
}

pub fn find_project_by_id(conn: &mut PgConnection, project_id: i32) -> QueryResult<Project> {
    projects::table.find(project_id).first::<Project>(conn)
}

macro_rules! filter_date_time {
    ($column:expr, $value:expr, $query:expr) => {
        if let Some(start) = $value.start {
            $query = if $value.include_start {
                $query.filter($column.ge(start))
            } else {
                $query.filter($column.gt(start))
            };
        }
        if let Some(end) = $value.end {
            $query = if $value.include_end {
                $query.filter($column.le(end))
            } else {
                $query.filter($column.lt(end))
            };
        }
    };
}

pub fn paginate_projects(
    conn: &mut PgConnection,
    user: &User,
    group: ProjectGroupFilterType,
    statuses: Option<Vec<ProjectUserStatusValue>>,
    search: Option<String>,
    is_archived: Option<bool>,
    created_at: Option<IntervalInType<DateTime<Utc>>>,
    updated_at: Option<IntervalInType<DateTime<Utc>>>,
    pagination: PaginationInType,
) -> ServiceResult<PaginationOutType<ProjectOutType>> {
    let statuses = statuses.unwrap_or(vec![
        ProjectUserStatusValue::Creator,
        ProjectUserStatusValue::Member,
    ]);
    let mut query = projects::table
        .inner_join(
            project_users::table.inner_join(
                project_user_statuses::table
                    .on(project_users::last_status_id.eq(project_user_statuses::id)),
            ),
        )
        .select(projects::all_columns)
        .order(projects::created_at.desc())
        .distinct()
        .into_boxed();
    if let Some(search) = search {
        let like_pattern = format!("{}%", search);
        query = query.filter(
            projects::name
                .ilike(like_pattern.to_owned())
                .or(projects::description.ilike(like_pattern.to_owned())),
        );
    }
    query = match group {
        ProjectGroupFilterType::Public => query.filter(projects::is_public.eq(true)),
        ProjectGroupFilterType::Private => query
            .filter(project_users::user_id.eq(user.id))
            .filter(project_user_statuses::status.eq_any(statuses)),
        ProjectGroupFilterType::Both => query.filter(
            project_users::user_id
                .eq(user.id)
                .and(project_user_statuses::status.eq_any(statuses))
                .or(projects::is_public.eq(true)),
        ),
    };
    if let Some(is_archived) = is_archived {
        query = query.filter(projects::is_archived.eq(is_archived))
    }
    if let Some(created_at) = created_at {
        filter_date_time!(projects::created_at, created_at, query);
    }
    if let Some(updated_at) = updated_at {
        filter_date_time!(projects::updated_at, updated_at, query);
    }
    let (projects, total_pages) = query
        .paginate(pagination.page as i64)
        .per_page(pagination.per_page as i64)
        .load_and_count_pages::<Project>(conn)
        .to_service_result()?;
    let mut data = vec![];
    for project in projects {
        data.push(ProjectOutType::from_project(conn, project)?);
    }
    Ok(PaginationOutType {
        data,
        total_pages: total_pages as i32,
    })
}

pub fn change_project(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    project_in: ProjectInType,
) -> ServiceResult<Project> {
    let project = find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_project(conn, &project, user.id, project_in.is_archived)?;
    let project_plugins =
        plugin_services::find_project_plugins(conn, project_id).to_service_result()?;
    for project_plugin in project_plugins {
        if let Some(vertex_value_type) = &project_plugin.vertex_value_type {
            if project_in.vertex_value_type != *vertex_value_type {
                return validation_error!(
                    "change_project_vertex_value_type_error",
                    plugin_name = project_plugin.name
                );
            }
        }
        if let Some(arc_value_type) = &project_plugin.arc_value_type {
            if project_in.arc_value_type != *arc_value_type {
                return validation_error!(
                    "change_project_arc_value_type_error",
                    plugin_name = project_plugin.name
                );
            }
        }
    }
    diesel::update(projects::table)
        .filter(projects::id.eq(&project_id))
        .set((
            projects::name.eq(project_in.name),
            projects::description.eq(project_in.description),
            projects::is_public.eq(project_in.is_public),
            projects::is_archived.eq(project_in.is_archived),
            projects::vertex_value_type.eq(project_in.vertex_value_type),
            projects::arc_value_type.eq(project_in.arc_value_type),
        ))
        .get_result::<Project>(conn)
        .to_service_result()
}

pub fn delete_project(conn: &mut PgConnection, user: &User, project_id: i32) -> ServiceResult<()> {
    permission_services::can_delete_project(conn, project_id, user.id)?;
    diesel::delete(projects::table.filter(projects::id.eq(project_id)))
        .execute(conn)
        .to_service_result()?;
    Ok(())
}

pub fn is_not_archived(project: &Project) -> ServiceResult<()> {
    if project.is_archived {
        return validation_error!("change_archived_project_error");
    }
    Ok(())
}

type ProjectIdWithUser = (
    i32,
    i32,
    String,
    String,
    String,
    bool,
    String,
    Option<String>,
    String,
    Option<String>,
    Option<String>,
    DateTime<Utc>,
    DateTime<Utc>,
);

impl ProjectOutType {
    pub fn from_project(conn: &mut PgConnection, project: Project) -> ServiceResult<Self> {
        Ok(ProjectOutType {
            id: project.id,
            name: project.name,
            description: project.description,
            creator: UserOutType::from(
                project_user_services::find_project_creator(conn, project.id)
                    .to_service_result()?,
            ),
            is_public: project.is_public,
            is_archived: project.is_archived,
            created_at: project.created_at,
            updated_at: project.updated_at,
            vertex_value_type: project.vertex_value_type,
            arc_value_type: project.arc_value_type,
            plugins: plugin_services::find_project_plugins(conn, project.id)
                .to_service_result()?
                .into_iter()
                .map(|plugin| plugin.name)
                .collect(),
        })
    }
    pub fn from_projects(
        conn: &mut PgConnection,
        projects: Vec<Project>,
    ) -> ServiceResult<Vec<Self>> {
        let mut creators = ProjectOutType::get_project_creators(conn, &projects)?;
        let mut plugins = ProjectOutType::get_project_plugins(conn, &projects)?;
        let mut result = vec![];
        for project in projects {
            let project_creator = ProjectOutType::find_project_creator(&mut creators, project.id);
            let project_plugins = ProjectOutType::find_project_plugins(&mut plugins, project.id);
            result.push(ProjectOutType {
                id: project.id,
                name: project.name,
                description: project.description,
                creator: UserOutType::from(project_creator),
                is_public: project.is_public,
                is_archived: project.is_archived,
                created_at: project.created_at,
                updated_at: project.updated_at,
                vertex_value_type: project.vertex_value_type,
                arc_value_type: project.arc_value_type,
                plugins: project_plugins.into_iter().rev().collect(),
            })
        }
        Ok(result)
    }
    fn get_project_creators(
        conn: &mut PgConnection,
        projects: &[Project],
    ) -> ServiceResult<Vec<ProjectIdWithUser>> {
        let project_ids = projects.iter().map(|project| project.id);
        project_user_statuses::table
            .inner_join(
                project_users::table
                    .inner_join(users::table)
                    .on(project_user_statuses::project_user_id.eq(project_users::id)),
            )
            .select((
                project_users::project_id,
                users::id,
                users::username,
                users::password,
                users::email,
                users::is_email_confirmed,
                users::first_name,
                users::second_name,
                users::last_name,
                users::avatar,
                users::language,
                users::created_at,
                users::updated_at,
            ))
            .filter(project_users::project_id.eq_any(project_ids))
            .filter(project_user_statuses::status.eq(ProjectUserStatusValue::Creator))
            .get_results::<ProjectIdWithUser>(conn)
            .to_service_result()
    }
    fn get_project_plugins(
        conn: &mut PgConnection,
        projects: &[Project],
    ) -> ServiceResult<Vec<(i32, String)>> {
        let project_ids = projects.iter().map(|project| project.id);
        projects::table
            .inner_join(project_plugins::table.inner_join(plugins::table))
            .select((projects::id, plugins::name))
            .filter(projects::id.eq_any(project_ids))
            .get_results::<(i32, String)>(conn)
            .to_service_result()
    }
    fn find_project_creator(creators: &mut Vec<ProjectIdWithUser>, project_id: i32) -> User {
        let index = creators
            .iter()
            .enumerate()
            .find(|(_, v)| v.0 == project_id)
            .unwrap()
            .0;
        let creator = creators.remove(index);
        return User {
            id: creator.1,
            username: creator.2,
            password: creator.3,
            email: creator.4,
            is_email_confirmed: creator.5,
            first_name: creator.6,
            second_name: creator.7,
            last_name: creator.8,
            avatar: creator.9,
            language: creator.10,
            created_at: creator.11,
            updated_at: creator.12,
        };
    }
    fn find_project_plugins(plugins: &mut Vec<(i32, String)>, project_id: i32) -> Vec<String> {
        let plugin_indices = plugins
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| *id == project_id)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let mut project_plugins = vec![];
        for index in plugin_indices.into_iter().rev() {
            project_plugins.push(plugins.remove(index).1)
        }
        project_plugins
    }
}
