use crate::forbidden_error;
use crate::models::Project;
use crate::response::ServiceResult;
use crate::services::{permission_services, project_services};
use diesel::PgConnection;

pub fn can_adjust_base(
    conn: &mut PgConnection,
    project_id: i32,
    user_id: i32,
) -> ServiceResult<bool> {
    permission_services::has_permission(conn, project_id, user_id, "can_adjust")
}

pub fn can_adjust(conn: &mut PgConnection, project: &Project, user_id: i32) -> ServiceResult<()> {
    if !can_adjust_base(conn, project.id, user_id)? {
        return forbidden_error!("can_adjust_forbidden_error");
    }
    project_services::is_not_archived(project)
}
