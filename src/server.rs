use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::types::{Client, ClientId};

pub struct Server {
    listener: Arc<TcpListener>,
    clients: Arc<Mutex<HashMap<ClientId, Client>>>,
}

impl Server {
    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).await.expect("Erro ao escutar");
        Server {
            listener: Arc::new(listener),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn listen(&self) {
        let clients = self.clients.clone();
        
        while let Ok((stream, _)) = self.listener.accept().await {
            let clients = clients.clone();
            tokio::spawn(async move {
                Server::handle_client(stream, clients).await;
            });
        }
    }

    async fn handle_client(stream: TcpStream, clients: Arc<Mutex<HashMap<ClientId, Client>>>) {
        let ws_stream = accept_async(stream).await.unwrap();
        let (write, read) = ws_stream.split();

        let client = Client::new(write, read);
        let client_id = client.id;

        clients.lock().await.insert(client_id, client.clone());

        println!("Cliente {} conectado", client_id);

        let mut reader = client.reader.lock().await;
        while let Some(Ok(msg)) = reader.next().await {
            if let Message::Text(txt) = msg {
                println!("Mensagem de {}: {}", client_id, txt);
            }
        }

        println!("Cliente {} desconectado", client_id);
        clients.lock().await.remove(&client_id);
    }

    pub async fn get_clients(&self) -> Vec<Client> {
        self.clients.lock().await.values().cloned().collect()
    }

    pub async fn broadcast(&self, msg: String) {
        let clients = self.clients.lock().await;
        for client in clients.values() {
            let writer = client.writer.clone();
            let msg = msg.clone();
            tokio::spawn(async move {
                let _ = writer.lock().await.send(Message::Text(msg)).await;
            });
        }
    }

    pub async fn send_to(&self, id: ClientId, msg: String) {
        if let Some(client) = self.clients.lock().await.get(&id) {
            let _ = client.writer.lock().await.send(Message::Text(msg)).await;
        }
    }
}
