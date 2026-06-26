use tokio::sync::Mutex;
use tokio::time::{Instant, Duration};

use std::sync::Arc;

use crate::types::{ClientId, TICK_RATE, WORLD_SIZE};
use crate::game::{GameManager};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MovePayload {
    pub thrust: bool,
    pub left: bool,
    pub right: bool,
    pub fire: bool,
}

/// Pacote com as constantes do jogo enviadas ao frontend.
/// O campo `type` permite ao cliente distinguir de outras mensagens.
#[derive(Serialize)]
pub struct GameInfo {
    #[serde(rename = "type")]
    msg_type: &'static str,
    tick_rate: u8,
    world_size: u32,
}

impl GameInfo {
    pub fn current() -> Self {
        Self {
            msg_type: "game_info",
            tick_rate: TICK_RATE,
            world_size: WORLD_SIZE,
        }
    }
}


#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ClientMessage {
    Move {
        #[serde(flatten)]
        data: MovePayload,
    },
    GetGameInfo,
    Ping,
}


pub enum WsResponse {
    Broadcast(String),
    Unicast(ClientId, String),
    Error(String),
    Nothing
}

#[derive(Clone)]
pub struct Router {
    game: Arc<Mutex<GameManager>>,
}

impl Router {
    pub fn new(game: Arc<Mutex<GameManager>>) -> Self {
        Self {
            game
        }
    }

    pub async fn handle_message(&self, client_id: &ClientId, message: &ClientMessage) -> WsResponse {
        match message {
            ClientMessage::Move{data} => {
                self.game.lock().await.handle_player_command(client_id, data);
                WsResponse::Nothing
            }

            ClientMessage::GetGameInfo => {
                let info = serde_json::to_string(&GameInfo::current()).unwrap_or_default();
                WsResponse::Unicast(client_id.clone(), info)
            }

            ClientMessage::Ping => {
                WsResponse::Unicast(client_id.clone(), "pong".to_string())
            }
        }
    }


    pub async fn handle_connect(&self, client_id: &ClientId) {
        let _ = self.game.lock().await.players.add_player(client_id);
    }

    pub async fn handle_disconnect(&self, client_id: &ClientId) {
        self.game.lock().await.players.rm_player(client_id);
    }

    pub async fn game_tick(&self,) -> String {
        let tick_duration = Duration::from_secs_f64(1.0 / TICK_RATE as f64);
        let t0 = Instant::now();

        self.game.lock().await.tick();
        let game_state = self.game.lock().await.get_game_state();
        //println!("{}", self.game.lock().await.game_info());
        
        let dt = Instant::now() - t0;
        tokio::time::sleep(tick_duration - dt).await;
        game_state
    }
}