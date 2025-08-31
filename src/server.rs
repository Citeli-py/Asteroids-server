use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tungstenite::client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::game::{self, GameManager};
use crate::types::{Client, ClientId, TICK_RATE};

#[derive(Clone)]
pub struct Server {
    listener: Arc<TcpListener>,
    clients: Arc<Mutex<HashMap<ClientId, Client>>>,
    game: Arc<Mutex<GameManager>>,
}

impl Server {
    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).await.expect("Erro ao escutar");
        Server {
            listener: Arc::new(listener),
            clients: Arc::new(Mutex::new(HashMap::new())),
            game: Arc::new(Mutex::new(GameManager::new()))
        }
    }

    pub async fn start(&self) {

        let self_clone = self.clone();
        tokio::spawn(async move {
            Server::listen(
                self_clone.game,
                self_clone.clients, 
                self_clone.listener
            ).await; // Escuta conexões em segundo plano
        });

        let tick_server = self.clone();
        let game_tick = self.game.clone();
        loop {
            tokio::time::sleep(Duration::from_secs_f64((1/TICK_RATE) as f64)).await;
            tick_server.broadcast(game_tick.lock().await.get_game_state().await).await;
        }
    }

    async fn listen(game: Arc<Mutex<GameManager>>, clients: Arc<Mutex<HashMap<ClientId, Client>>>, listener: Arc<TcpListener>) {
    
        while let Ok((stream, _)) = listener.accept().await {
            let clients = clients.clone();
            let game = game.clone();
            tokio::spawn(async move {
                Server::handle_client(stream, clients, game).await;
            });
        }
    }

    async fn handle_client(stream: TcpStream, clients: Arc<Mutex<HashMap<ClientId, Client>>>, game: Arc<Mutex<GameManager>>) {
        
        let client = Server::on_connect(game.clone(), clients.clone(), stream).await;

        let mut reader = client.reader.lock().await;
        while let Some(Ok(msg)) = reader.next().await {
            if let Message::Text(txt) = msg {
                Server::on_message(&client, game.clone(), txt).await;
            }
        }

        Server::on_disconnect(game.clone(), clients.clone(), &client).await;
    }

    async fn on_connect(game: Arc<Mutex<GameManager>>, clients: Arc<Mutex<HashMap<ClientId, Client>>>, stream: TcpStream) -> Client{

        let ws_stream = accept_async(stream).await.unwrap();
        let (write, read) = ws_stream.split();
        let client = Client::new(write, read);
        let client_id = client.id;

        clients.lock().await.insert(client_id, client.clone());
        game.lock().await.add_player(&client_id).await;

        println!("Cliente {} conectado", client_id);
        let _ = client.writer.lock().await.send(Message::Text(format!("connected:{}", client_id))).await;

        return client.clone();
    }

    async fn on_message(client: &Client, game: Arc<Mutex<GameManager>>, message: String) {
        println!("Mensagem de {}: {}", client.id, message);
        game.lock().await.handle_player_command(&(client.id), &message).await;
    }

    async fn on_disconnect(game: Arc<Mutex<GameManager>>, clients: Arc<Mutex<HashMap<ClientId, Client>>>, client: &Client) {
        println!("Cliente {} desconectado", client.id);
        clients.lock().await.remove(&(client.id));
        game.lock().await.rm_player(&(client.id)).await;
    }

    pub async fn get_clients(&self) -> Vec<Client> {
        self.clients.lock().await.values().cloned().collect()
    }

    pub async fn broadcast(&self, msg: String) {

        println!("Broadcasting: {}", msg);

        let clients = self.clients.lock().await;
        for client in clients.values() {
            let writer = client.writer.clone();
            let id = client.id;

            let msg = msg.clone();
            tokio::spawn(async move {
                let result = writer.lock().await.send(Message::Text(msg)).await;
                if result.is_err() {
                    println!("\tError broadcasting msg to {}", id);
                }
            });
        }
    }

    pub async fn send_to(&self, id: ClientId, msg: String) {
        if let Some(client) = self.clients.lock().await.get(&id) {
            let result = client.writer.lock().await.send(Message::Text(msg)).await;

            if result.is_err() {
                println!("\tError sending message to {}", client.id);
            }
        }
    }
}
