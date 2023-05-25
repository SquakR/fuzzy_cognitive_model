use super::listener::AdjustmentRunConnections;
use crate::plugins::adjustment::types::{AdjustmentRunActionErrorType, AdjustmentRunActionType};
use schemars::JsonSchema;
use serde::Serialize;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message};

#[derive(Clone)]
pub struct WebSocketAdjustmentRunService {
    adjustment_run_connections: AdjustmentRunConnections,
}

impl WebSocketAdjustmentRunService {
    pub fn new(adjustment_run_connections: AdjustmentRunConnections) -> Self {
        Self {
            adjustment_run_connections,
        }
    }
    pub async fn disconnect_sessions(&self, session_ids: &[i32]) -> () {
        for (_, senders) in self.adjustment_run_connections.lock().await.iter_mut() {
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
            .adjustment_run_connections
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
    pub async fn notify<T>(&self, adjustment_run_action: AdjustmentRunActionType<T>) -> ()
    where
        T: Clone + Serialize + JsonSchema,
    {
        let message = Message::Text(serde_json::to_string(&adjustment_run_action).unwrap());
        self.send_message(message, adjustment_run_action.project_id)
            .await;
    }
    pub async fn notify_error(&self, adjustment_run_action: AdjustmentRunActionErrorType) -> () {
        let message = Message::Text(serde_json::to_string(&adjustment_run_action).unwrap());
        self.send_message(message, adjustment_run_action.project_id)
            .await;
    }
    pub async fn send_message(&self, message: Message, project_id: i32) -> () {
        for connection_sender in self
            .adjustment_run_connections
            .lock()
            .await
            .get_mut(&project_id)
            .unwrap_or(&mut vec![])
        {
            connection_sender.sender.send(message.clone()).unwrap()
        }
    }
}
