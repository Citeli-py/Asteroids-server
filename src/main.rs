mod game;
mod server;
mod types;

use std::time::Duration;
use server::Server;
use tokio::main;

#[main]
async fn main() {
    let server = Server::new("127.0.0.1:8080").await;
    server.start().await; // Inicia a escuta em segundo plano

    let ping_server = server.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            ping_server.broadcast("ping".to_string()).await;
        }
    });

    tokio::time::sleep(Duration::from_secs(2)).await;
    let cliente = &server.get_clients().await[0];

    server.send_to(cliente.id, "fala cliente".to_string()).await;

    tokio::time::sleep(Duration::from_secs(100)).await;
}
