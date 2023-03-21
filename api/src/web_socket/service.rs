use serde::Serialize;
use tokio_tungstenite::tungstenite::protocol::Message;

#[rocket::async_trait]
pub trait WebSocketService<T: Serialize + Send> {
    async fn notify(&self, id: i32, action: String, data: T) -> ()
    where
        T: 'async_trait,
    {
        let notification_data = NotificationData::new(action, data);
        let message = Message::Text(serde_json::to_string(&notification_data).unwrap());
        self.send_message(id, message).await;
    }
    async fn send_message(&self, id: i32, message: Message) -> ();
}

#[derive(Serialize)]
struct NotificationData<T: Serialize + Send> {
    action: String,
    data: T,
}

impl<T> NotificationData<T>
where
    T: Serialize + Send,
{
    fn new(action: String, data: T) -> NotificationData<T> {
        NotificationData { action, data }
    }
}
