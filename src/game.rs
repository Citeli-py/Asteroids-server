use crate::types::{ClientId};
use crate::player::{Player, CMD};

use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct GameManager {
    players: Arc<Mutex<HashMap<ClientId, Player>>>,
}

impl GameManager {
    pub fn new() -> Self {
        let players = Arc::new(Mutex::new(HashMap::new()));
        Self { players }
    }

    pub async fn add_player(&mut self, client_id: &ClientId) {
        println!("New player");
        let new_player = Player::new(client_id);

        self.players.lock().await.insert(*client_id, new_player);
    }

    pub async fn rm_player(&mut self, client_id: &ClientId) {
        self.players.lock().await.remove(client_id);
    }

    pub async fn handle_player_command(&mut self, client_id: &ClientId, player_command: &String) {
        /*
        UP
        LEFT
        RIGHT
        SHOT
        */
        let mut players = self.players.lock().await;

        if let Some(player) = players.get_mut(client_id) {

            if player_command.contains("UP") {
                player.push_command(CMD::UP);
            }
    
            if player_command.contains("LEFT") {
                player.push_command(CMD::LEFT);
            }
    
            if player_command.contains("RIGHT") {
                player.push_command(CMD::RIGHT);
            }

        }
        

        
    }

    pub async fn get_game_state(&self, ) -> String {

        let mut players = self.players.lock().await;
        let mut game_state = String::from("{\"Players\":[");

        let mut comma = "";

        for player in players.values_mut() {

            player.update();

            let player_str = format!("{} {}", comma, player.to_json());
            game_state.push_str(&player_str);
            comma = ",";
        }

        game_state.push_str("]}");
        game_state
    }
}
