use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;


use std::{collections::HashMap, sync::Arc};

use crate::networking::router::{Router, ClientMessage, WsResponse};
use crate::types::{ClientId};
use crate::networking::client::Client;

type ClientMap = Arc<Mutex<HashMap<ClientId, Client>>>;

#[derive(Clone)]
pub struct WebSocketHandler {
    clients: ClientMap,
    router: Router
}

impl WebSocketHandler {
    pub fn new(router: Router) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            router
        }
    }

    pub async fn get_clients(&self) -> Vec<Client> {
        self.clients.lock().await.values().cloned().collect()
    }

    pub async  fn on_connect(&self, sender: SplitSink<WebSocket, Message>) -> ClientId {
        let client = Client::new(sender);

        self.clients.lock().await.insert(client.id, client.clone());
        println!("Cliente {} conectado", client.id);
        self.router.handle_connect(&client.id).await;
        self.unicast(&client.id, format!("connected:{}", client.id)).await;

        client.id
    }

    pub async fn on_message(&self, client_id: &ClientId, message: Message) {
        if let Message::Text(txt) = message {
            if let Ok(payload) = serde_json::from_str::<ClientMessage>(&txt) {
                let response = self.router.handle_message(client_id, &payload).await;
                self.handle_response(client_id, response).await;
            }
        }
    }

    async fn handle_response(&self, client_id: &ClientId, response: WsResponse) {
         match response {
            WsResponse::Unicast(id, msg)  => self.unicast(&id, msg).await,
            WsResponse::Broadcast(msg)          => self.broadcast(msg).await,
            WsResponse::Error(msg)              => self.unicast(client_id, msg).await,
            WsResponse::Nothing                         => {}
        }
    }

    pub async fn on_disconnect(&self, client_id: &ClientId) {
        self.clients.lock().await.remove(&client_id);
        self.router.handle_disconnect(client_id).await;
        println!("Cliente {} desconectado", client_id);
    }

    pub async fn handle_socket(self: Arc<Self>, socket: WebSocket) {
        let (sender, mut receiver) = socket.split();

        let client_id = self.on_connect(sender).await;

        // loop de mensagens
        while let Some(Ok(msg)) = receiver.next().await {
            self.on_message(&client_id, msg).await;
        }

        // on_disconnect
        self.on_disconnect(&client_id).await;
    }

    pub async fn broadcast(&self, msg: String) {
        let clients = self.clients.lock().await;

        for client in clients.values() {
            let sender = client.sender.clone();
            let msg = msg.clone();

            tokio::spawn(async move {
                let _ = sender.lock().await.send(Message::Text(msg.into())).await;
            });
        }
    }

    pub async fn unicast(&self, client_id: &ClientId, msg: String) {
        let clients = self.clients.lock().await;

        if let Some(client) = clients.get(client_id) {
            let _ = client.sender.lock().await.send(Message::Text(msg.into())).await;
        }
    }

    pub async fn start(self: Arc<Self>) {
        loop {
            self.broadcast(
                self.router.game_tick().await
            ).await;
        }

    }

}
