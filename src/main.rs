mod game;
mod server;
mod types;
mod player;
mod bullet;
mod collision_object;
mod bullet_collection;

use std::{sync::Arc, time::Duration};
use types::TICK_RATE;

use futures_util::lock::Mutex;
use server::Server;
use tokio::main;

use crate::game::GameManager;

#[main]
async fn main() {

    let game = Arc::new(Mutex::new(GameManager::new()));
    let mut server = Server::new("0.0.0.0:8080").await;


    let game_connect = game.clone();
    server.set_on_connect( move |client, _| {
        let id = client.id.clone();
        let game_connect = game_connect.clone();

        async move {
            game_connect.lock().await.add_player(&id);
        }
    }).await;

    let game_message = game.clone();
    server.set_on_message( move |client, message| {
        let id = client.id.clone();
        let msg = message.clone();
        let game_message = game_message.clone();

        async move {
            game_message.lock().await.handle_player_command(&id, &msg);
        }
    }).await;

    let game_disconnect = game.clone();
    server.set_on_disconnect( move |client, _| {
        let id = client.id.clone();
        let game_disconnect = game_disconnect.clone();

        async move {
            game_disconnect.lock().await.rm_player(&id);
        }
    }).await;


    let mut listen_server = server.clone();
    tokio::spawn(async move {
            listen_server.listen().await;
        }
    );

    let tick_server = server.clone();
    let game_tick = game.clone();
    loop {
        let dt = 1.0/TICK_RATE as f64;
        tokio::time::sleep(Duration::from_secs_f64(dt)).await;
        let game_state = game_tick.lock().await.get_game_state();
        tick_server.broadcast(game_state).await;
    }

}
