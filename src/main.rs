mod game;
mod server;
mod types;
mod player;

use std::sync::Arc;

use futures_util::lock::Mutex;
use server::Server;
use tokio::main;

use crate::game::GameManager;

#[main]
async fn main() {

    let game = Arc::new(Mutex::new(GameManager::new()));
    let mut server = Server::new("127.0.0.1:8080").await;

    server.set_on_message( move |client, message| {
        let id = client.id.clone();
        let msg = message.clone();
        let game_message = game.clone();

        async move {
            game_message.lock().await.handle_player_command(&id, &msg).await;
            println!("Callback: cliente {} mandou mensagem!", id);
        }
    }).await;


    server.listen().await;

}
