mod game;
mod server;
mod types;
mod player;

use server::Server;
use tokio::main;

#[main]
async fn main() {

    let server = Server::new("127.0.0.1:8080").await;
    server.start().await; // Inicia a escuta em segundo plano
}
