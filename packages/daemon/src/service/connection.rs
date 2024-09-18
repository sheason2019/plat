use axum::extract::ws::Message;
use tokio::sync::broadcast::{Receiver, Sender};
use uuid::Uuid;

pub struct Connection {
    pub access_token: String,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Connection {
    pub fn new(sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Connection {
            access_token: Uuid::new_v4().to_string(),
            sender,
            receiver,
        }
    }
}
