use super::web_socket_listener::ProjectConnections;
use crate::models::User;
use crate::response::{ServiceResult, ToServiceResult};
use crate::services::{permission_services, project_services, user_services};
use diesel::PgConnection;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message};

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
    pub async fn get_active_users(
        &self,
        conn: &mut PgConnection,
        user: &User,
        project_id: i32,
    ) -> ServiceResult<Vec<User>> {
        let project = project_services::find_project_by_id(conn, project_id)
            .to_service_result_find(String::from("project_not_found_error"))?;
        permission_services::can_view_project(conn, &project, user)?;
        user_services::find_users_by_id(
            conn,
            self.project_connections
                .lock()
                .await
                .get(&project_id)
                .unwrap_or(&vec![])
                .iter()
                .map(|sender| sender.data.user_id),
        )
        .to_service_result()
    }
    pub async fn disconnect_sessions(&self, session_ids: &[i32]) -> () {
        for (_, senders) in self.project_connections.lock().await.iter_mut() {
            for connection_sender in senders
                .iter_mut()
                .filter(|sender| session_ids.contains(&sender.data.session_id))
            {
                connection_sender
                    .sender
                    .send(Message::Close(Some(CloseFrame {
                        code: CloseCode::Normal,
                        reason: "The user session has ended.".into(),
                    })))
                    .await
                    .unwrap();
            }
        }
    }
    pub async fn disconnect_project(&self, project_id: i32) -> () {
        for project_connection in self
            .project_connections
            .lock()
            .await
            .get_mut(&project_id)
            .unwrap_or(&mut vec![])
        {
            project_connection
                .sender
                .send(Message::Close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: "The project has been deleted.".into(),
                })))
                .await
                .unwrap()
        }
    }
}
