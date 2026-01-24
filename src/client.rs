use axum::extract::ws::WebSocket;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use axum::extract::ws::Message;
use uuid::Uuid;

use std::sync::Arc;
use futures_util::lock::Mutex;

use crate::types::ClientId;

#[derive(Clone)]
pub struct Client {
    pub id: ClientId,
    pub sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
}

impl Client {
    pub fn new(sender: SplitSink<WebSocket, Message>) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender: Arc::new(Mutex::new(sender)),
        }
    }
}
