use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::{sleep};

use std::thread;

use tokio_tungstenite::WebSocketStream;
use futures_util::stream::SplitStream;
use futures_util::stream::SplitSink;


type WSStream = WebSocketStream<TcpStream>;
type ArcMutexReader = Arc<Mutex<SplitStream<WSStream>>>;
type ArcMutexWriter = Arc<Mutex<SplitSink<WSStream, Message>>>;


#[derive(Clone, Debug)]
struct Client {
    id: Uuid,
    x: f32,
    y: f32,
    writer: ArcMutexWriter,
    reader: ArcMutexReader,
}

struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<Vec<Client>>>, // Mutex adicionado
}

impl Server {
    async fn new(_port: &str) -> Server {
        let ip = format!("127.0.0.1:{}", _port);
        let listener = TcpListener::bind(ip)
            .await
            .expect("Erro ao escutar na porta");

        Server {
            listener,
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn listen(&self) {

        let clients = self.clients.clone();
        tokio::spawn(async move {
            Server::listen_messages(clients).await;
        });

        while let Ok((stream, _)) = self.listener.accept().await {
            let clients = self.clients.clone(); // clona o Arc
            tokio::spawn(async move {
                Server::get_clients(stream, clients).await;
            });
        }

    }

    fn ping(client: Client, write: ArcMutexWriter) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(5)).await;

                let mut writer = write.lock().await;
                if writer.send(Message::Text("ping".to_string())).await.is_err() {
                    println!("Erro ao enviar ping para {}", client.id);
                    break;
                }

                println!("Ping enviado para {}", client.id);
            }
        });
    }


    async fn get_clients(stream: TcpStream, clients: Arc<Mutex<Vec<Client>>>) {
        let ws_stream = accept_async(stream)
            .await
            .expect("Erro ao aceitar conexão WebSocket");

        println!("Novo cliente conectado!");

        let ( write, read) = ws_stream.split();
        let  write= Arc::new(Mutex::new(write));
        let read = Arc::new(Mutex::new(read));

        let new_client = Client {
            id: Uuid::new_v4(),
            x: 0.0,
            y: 0.0,
            writer: write.clone(),
            reader: read.clone()
        };

        // Adiciona o cliente
        clients.lock().await.push(new_client.clone());
        Server::ping(new_client.clone(), write.clone());
    }


    async fn listen_messages(clients: Arc<Mutex<Vec<Client>>>) {

        loop {
            for client in clients.lock().await.iter()  {
                if let Some(Ok(msg)) = client.reader.lock().await.next().await {
                    if let Message::Text(texto) = msg {
                        println!("Mensagem recebida: {}", texto);

                        if texto == "pong" {
                            println!("Pong recebido de {}", client.id);
                        }

                        let mensagem = format!("Seu id é {}", client.id);
                        let _ = client.writer.lock().await
                            .send(Message::Text(mensagem.into()))
                            .await;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let server = Server::new("8080").await;
    server.listen().await;
}
