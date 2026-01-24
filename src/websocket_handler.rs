use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio::time::{Instant, Duration};

use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};

use crate::types::{ClientId, TICK_RATE};
use crate::client::Client;
use crate::game::{self, GameManager};

type ClientMap = Arc<Mutex<HashMap<ClientId, Client>>>;

#[derive(Clone)]
pub struct WebSocketHandler {
    clients: ClientMap,
    game: Arc<Mutex<GameManager>>,
}

impl WebSocketHandler {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            game: Arc::new(Mutex::new(GameManager::new())),
        }
    }

    pub async fn get_clients(&self) -> Vec<Client> {
        self.clients.lock().await.values().cloned().collect()
    }

    pub async  fn on_connect(&self, sender: SplitSink<WebSocket, Message>) -> Uuid {
        let client = Client::new(sender);

        self.clients.lock().await.insert(client.id, client.clone());

        // on_connect
        self.game.lock().await.players.add_player(&client.id).ok();
        println!("Cliente {} conectado", client.id);
        let _ = client.sender.lock().await.send(Message::Text(format!("connected:{}", client.id).into())).await;

        client.id
    }

    pub async fn on_message(&self, client_id: &Uuid, message: Message) {
        if let Message::Text(txt) = message {
            self.game
                .lock()
                .await
                .handle_player_command(&client_id, &txt.to_string());
        }
    }

    pub async fn on_disconnect(&self, client_id: &Uuid) {
        self.clients.lock().await.remove(&client_id);
        self.game.lock().await.players.rm_player(&client_id);
        println!("Cliente {} desconectado", client_id);
    }

    pub async fn handle_socket(self: Arc<Self>, socket: WebSocket) {
        let (mut sender, mut receiver) = socket.split();

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

    pub async fn start(self: Arc<Self>) {

        let tick_duration = Duration::from_secs_f64(1.0 / TICK_RATE as f64);

        loop {
            let t0 = Instant::now();

            self.game.lock().await.tick();
            let game_state = self.game.lock().await.get_game_state();
            
            let dt = Instant::now() - t0;
            tokio::time::sleep(tick_duration - dt).await;

            self.broadcast(game_state).await;
        }

    }

}
