use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tungstenite::client;
use tungstenite::handshake::server;
use std::collections::HashMap;
use std::sync::Arc;
use std::pin::Pin;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::game::{self, GameManager};
use crate::types::{Client, ClientId, TICK_RATE};

type AsyncCallback = Box<
    dyn Fn(&Client, &String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
>;

#[derive(Clone)]
pub struct Server {
    listener: Arc<TcpListener>,
    clients: Arc<Mutex<HashMap<ClientId, Client>>>,
    game: Arc<Mutex<GameManager>>,
    on_message_callback: Arc<Mutex<Option<AsyncCallback>>>,
    on_connect_callback: Arc<Mutex<Option<AsyncCallback>>>,
}


impl Server {
    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).await.expect("Erro ao escutar");
        Server {
            listener: Arc::new(listener),
            clients: Arc::new(Mutex::new(HashMap::new())),
            game: Arc::new(Mutex::new(GameManager::new())),
            on_connect_callback: Arc::new(Mutex::new(None)),
            on_message_callback: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&mut self) {

        // let self_clone = self.clone();
        // tokio::spawn(async move {
        //     Server::listen(
        //         self_clone.game,
        //         self_clone.clients, 
        //         self_clone.listener,
        //     ).await; // Escuta conexões em segundo plano
        // });

        // let tick_server = self.clone();
        // let game_tick = self.game.clone();
        // loop {
        //     tokio::time::sleep(Duration::from_secs_f64((1/TICK_RATE) as f64)).await;
        //     tick_server.broadcast(game_tick.lock().await.get_game_state().await).await;
        // }
        
    }

    pub async fn listen(&mut self,) {
    
        while let Ok((stream, _)) = self.listener.accept().await {
            let server = self.clone();
            tokio::spawn(async move {
                Server::handle_client(server, stream).await;
            });
        }
    }

    async fn handle_client(server: Server, stream: TcpStream) {
        let game = server.game.clone();
        let clients = server.clients.clone();

        let client = Server::on_connect(game.clone(), clients.clone(), stream, server.on_connect_callback.clone()).await;

        let mut reader = client.reader.lock().await;
        while let Some(Ok(msg)) = reader.next().await {
            if let Message::Text(txt) = msg {
                Server::on_message(&client, game.clone(), txt, server.on_message_callback.clone()).await;
            }
        }

        Server::on_disconnect(game.clone(), clients.clone(), &client).await;
    }


    async fn on_connect(game: Arc<Mutex<GameManager>>, clients: Arc<Mutex<HashMap<ClientId, Client>>>, stream: TcpStream, callback: Arc<Mutex<Option<AsyncCallback>>>) -> Client {

        let ws_stream = accept_async(stream).await.unwrap();
        let (write, read) = ws_stream.split();
        let client = Client::new(write, read);
        let client_id = client.id;

        clients.lock().await.insert(client_id, client.clone());

        println!("Cliente {} conectado", client_id);
        let _ = client.writer.lock().await.send(Message::Text(format!("connected:{}", client_id))).await;

        if let Some(cb) = &*callback.lock().await {
            (cb)(&client, &String::from("")).await;
        }

        return client.clone();
    }

    async fn on_message(client: &Client, game: Arc<Mutex<GameManager>>, message: String, callback: Arc<Mutex<Option<AsyncCallback>>>) {
        // chama callback se existir
        if let Some(cb) = &*callback.lock().await {
            (cb)(&client, &message).await; 
        }

        println!("Mensagem de {}: {}", client.id, message);
        game.lock().await.handle_player_command(&(client.id), &message).await;
    }


    async fn on_disconnect(game: Arc<Mutex<GameManager>>, clients: Arc<Mutex<HashMap<ClientId, Client>>>, client: &Client) {
        println!("Cliente {} desconectado", client.id);
        clients.lock().await.remove(&(client.id));
        game.lock().await.rm_player(&(client.id)).await;
    }

    
    pub async fn set_on_message<F, Fut>(&mut self, callback: F) 
    where 
        F: Fn(&Client, &String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut cb = self.on_message_callback.lock().await; 
        *cb = Some(Box::new(move |client, message| Box::pin(callback(client, message))));
    }

    pub async fn set_on_connect<F, Fut>(&mut self, callback: F) 
    where 
        F: Fn(&Client, &String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut cb = self.on_connect_callback.lock().await; 
        *cb = Some(Box::new(move |client, message| Box::pin(callback(client, message))));
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
