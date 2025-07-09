mod game;
mod server;
mod types;

use server::Server;
use tokio::main;

#[main]
async fn main() {
    let mut server = Server::new("127.0.0.1:8080").await;
    server.listen().await;
}
