use crate::types::ClientId;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone)]
pub struct Player {
    pos: Position,
    client_id: ClientId,
}

#[derive(Clone)]
pub struct GameManager {
    players: Arc<Mutex<HashMap<ClientId, Player>>>,
}

impl GameManager {
    pub fn new() -> Self {
        let players = Arc::new(Mutex::new(HashMap::new()));
        Self { players }
    }

    pub async fn update_position(&self, pos_text: &String, client_id: &ClientId) {
        let pos_text_xy: Vec<&str> = pos_text.split(':').collect();
        if pos_text_xy.len() != 2 {
            panic!("Formato inválido para pos_text: {}", pos_text);
        }

        let pos_xy: Vec<&str> = pos_text_xy[1].split(',').collect();
        if pos_xy.len() != 2 {
            panic!("Formato inválido de coordenadas: {}", pos_text_xy[1]);
        }

        let x = pos_xy[0].trim().parse::<f32>().expect("Erro ao converter X");
        let y = pos_xy[1].trim().parse::<f32>().expect("Erro ao converter Y");

        let mut players = self.players.lock().await;

        if let Some(player) = players.get_mut(client_id) {
            player.pos = Position { x, y };
            println!("Player: {} at ({}, {})", client_id, x, y);
        }
    }

    pub async fn add_player(&mut self, client_id: &ClientId) {
        println!("New player");
        let new_player = Player {
            pos: Position { x: 0.0, y: 0.0 },
            client_id: *client_id,
        };

        self.players.lock().await.insert(*client_id, new_player);
    }

    pub async fn rm_player(&mut self, client_id: &ClientId) {
        self.players.lock().await.remove(client_id);
    }

    pub async fn handle_player_command(&mut self, client_id: &ClientId, player_command: &String) {
        /*
        pos: x, y, vx, vy, theta
        shot!
        */
        if player_command.contains("pos:") {
            self.update_position(player_command, client_id).await;
        }
    }

    pub async fn get_game_state(&self, ) -> String {

        let players = self.players.lock().await.clone();
        let mut game_state = String::from("{\"Players\":[");

        let mut comma = "";

        for player in players.values() {

            let player_str = format!("{}{{ \"id\":\"{}\", \"x\": {}, \"y\":{} }}", comma, player.client_id, player.pos.x, player.pos.y);
            game_state.push_str(&player_str);
            comma = ",";
        }

        game_state.push_str("]}");
        game_state
    }
}
