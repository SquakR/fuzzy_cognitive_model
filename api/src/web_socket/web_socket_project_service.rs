use super::web_socket_listener::ProjectConnections;
use crate::models::User;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::services::{permission_services, project_services, user_services};
use diesel::PgConnection;

#[derive(Clone)]
pub struct WebSocketProjectService {
    project_connections: ProjectConnections,
}

impl WebSocketProjectService {
    pub fn new(project_connections: ProjectConnections) -> Self {
        WebSocketProjectService {
            project_connections,
        }
    }
    pub fn get_active_users(
        &self,
        conn: &mut PgConnection,
        user: &User,
        project_id: i32,
    ) -> ServiceResult<Vec<User>> {
        let project = project_services::find_project_by_id(conn, project_id)
            .to_service_result_find(String::from("project_not_found_error"))?;
        if !permission_services::can_view_project(conn, user, &project)? {
            return Err(AppError::ForbiddenError(String::from(
                "view_project_forbidden_error",
            )));
        }
        user_services::find_users_by_id(
            conn,
            self.project_connections
                .lock()
                .unwrap()
                .get(&project_id)
                .unwrap_or(&vec![])
                .iter()
                .map(|sender| sender.data.user_id),
        )
        .to_service_result()
    }
}
