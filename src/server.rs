use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::pin::Pin;
use crate::types::{Client, ClientId};

type AsyncCallback = Box<
    dyn Fn(&Client, &String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
>;

type ClientMap = Arc<Mutex<HashMap<ClientId, Client>>>;

#[derive(Clone)]
pub struct Server {
    listener: Arc<TcpListener>,
    clients: ClientMap,
    on_message_callback: Arc<Mutex<Option<AsyncCallback>>>,
    on_connect_callback: Arc<Mutex<Option<AsyncCallback>>>,
    on_disconnect_callback: Arc<Mutex<Option<AsyncCallback>>>,
}


impl Server {
    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).await.expect("Erro ao escutar");
        Server {
            listener: Arc::new(listener),
            clients: Arc::new(Mutex::new(HashMap::new())),
            on_connect_callback: Arc::new(Mutex::new(None)),
            on_message_callback: Arc::new(Mutex::new(None)),
            on_disconnect_callback: Arc::new(Mutex::new(None))
        }
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
        let clients = server.clients.clone();

        let client = Server::on_connect(clients.clone(), stream, server.on_connect_callback.clone()).await;

        let mut reader = client.reader.lock().await;
        while let Some(Ok(msg)) = reader.next().await {
            if let Message::Text(txt) = msg {
                Server::on_message(&client,txt, server.on_message_callback.clone()).await;
            }
        }

        Server::on_disconnect(clients.clone(), &client, server.on_disconnect_callback.clone()).await;
    }


    async fn on_connect(clients: ClientMap, stream: TcpStream, callback: Arc<Mutex<Option<AsyncCallback>>>) -> Client {

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

    async fn on_message(client: &Client, message: String, callback: Arc<Mutex<Option<AsyncCallback>>>) {
        // chama callback se existir
        if let Some(cb) = &*callback.lock().await {
            (cb)(&client, &message).await; 
        }

        println!("Mensagem de {}: {}", client.id, message);
    }


    async fn on_disconnect(clients: ClientMap, client: &Client, callback: Arc<Mutex<Option<AsyncCallback>>>) {
        println!("Cliente {} desconectado", client.id);
        clients.lock().await.remove(&(client.id));
        
        if let Some(cb) = &*callback.lock().await {
            (cb)(&client, &String::from("")).await; 
        }
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

    pub async fn set_on_disconnect<F, Fut>(&mut self, callback: F) 
    where 
        F: Fn(&Client, &String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut cb = self.on_disconnect_callback.lock().await; 
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
