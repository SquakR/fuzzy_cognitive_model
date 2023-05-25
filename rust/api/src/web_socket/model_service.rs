use super::listener::ModelConnections;
use crate::models::User;
use crate::response::{ServiceResult, ToServiceResult};
use crate::services::{permission_services, project_services, user_services};
use crate::types::ModelActionType;
use diesel::PgConnection;
use schemars::JsonSchema;
use serde::Serialize;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message};

#[derive(Clone)]
pub struct WebSocketModelService {
    model_connections: ModelConnections,
}

impl WebSocketModelService {
    pub fn new(model_connections: ModelConnections) -> Self {
        Self { model_connections }
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
            self.model_connections
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
        for (_, senders) in self.model_connections.lock().await.iter_mut() {
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
                    .unwrap();
            }
        }
    }
    pub async fn disconnect_project(&self, project_id: i32) -> () {
        for project_connection in self
            .model_connections
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
                .unwrap()
        }
    }
    pub async fn notify<T>(&self, model_action: ModelActionType<T>) -> ()
    where
        T: Clone + Serialize + JsonSchema,
    {
        let message = Message::Text(serde_json::to_string(&model_action).unwrap());
        self.send_message(message, model_action.project_id).await;
    }
    pub async fn send_message(&self, message: Message, project_id: i32) -> () {
        for connection_sender in self
            .model_connections
            .lock()
            .await
            .get_mut(&project_id)
            .unwrap_or(&mut vec![])
        {
            connection_sender.sender.send(message.clone()).unwrap()
        }
    }
}
