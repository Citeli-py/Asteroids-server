mod game;
mod websocket_handler;
mod types;
mod player;
mod bullet;
mod collision_object;
mod warp_object;
mod bullet_collection;
mod player_collection;
mod asteroid_collection;
mod asteroid;
mod client;

use game::GameManager;
use websocket_handler::WebSocketHandler;
use types::TICK_RATE;

use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade},
    response::IntoResponse,
};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let game = Arc::new(Mutex::new(GameManager::new()));
    let server = Arc::new(WebSocketHandler::new());

    // game loop
    {
        let server = server.clone();
        let game = game.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs_f64(1.0 / TICK_RATE as f64)).await;
                game.lock().await.tick();
                let state = game.lock().await.get_game_state();
                server.broadcast(state).await;
            }
        });
    }

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/ws", get(move |ws: WebSocketUpgrade| {
            let server = server.clone();
            async move {
                ws.on_upgrade(move |socket| async move {
                    server.handle_socket(socket).await;
                })
            }
        }));

    let port: u16 = std::env::var("PORT").unwrap_or("8080".into()).parse().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

