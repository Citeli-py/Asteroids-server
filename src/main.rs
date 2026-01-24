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

use websocket_handler::WebSocketHandler;
use std::sync::Arc;

use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade},
    http::StatusCode
};
use std::net::SocketAddr;

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}


#[tokio::main]
async fn main() {

    let server = Arc::new(WebSocketHandler::new());

    {
        let broadcast_server = Arc::clone(&server);
        tokio::spawn(async move {
            broadcast_server.start().await
        });
    }

    let app = Router::new()
        .route("/health", get(health_check))
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

