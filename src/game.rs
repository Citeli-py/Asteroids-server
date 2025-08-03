use crate::types::{ClientId, TICK_RATE};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;


#[derive(PartialEq, Clone, Debug)]
enum CMD {
    UP,
    LEFT,
    RIGHT,
    SHOT,
    NONE
}

#[derive(Clone)]
pub struct Player {
    x: f32,
    y: f32,
    angle: f32,
    vx: f32,
    vy: f32,
    turn_speed: f32,
    acceleration: f32,
    friction: f32,
    last_command: CMD,
    client_id: ClientId,
}

impl Player {
    
    pub fn new(client_id: &ClientId) -> Player{
        Player {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            turn_speed: 0.2,
            acceleration: 0.2,
            friction: 0.999,
            last_command: CMD::NONE,
            client_id: client_id.clone()
        }
    }

    pub fn movement(&mut self,) {
        
        let dt = 1.0/TICK_RATE as f32;

        if self.last_command == CMD::UP {
            self.vx += self.acceleration * f32::cos(self.angle) * dt;
            self.vy += self.acceleration * f32::sin(self.angle) * dt;
        }

        if self.last_command == CMD::LEFT {
            self.angle -= self.turn_speed*dt;
        }

        if self.last_command == CMD::RIGHT {
            self.angle += self.turn_speed*dt;
        }

        self.x += self.vx*dt;
        self.y += self.vy*dt; 

        self.vx *= self.friction;
        self.vy *= self.friction;


        self.last_command = CMD::NONE;

    }
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
        let mut player_move: CMD = CMD::NONE;
        if player_command.contains("UP") {
            player_move = CMD::UP;
        }

        if player_command.contains("LEFT") {
            player_move = CMD::LEFT;
        }

        if player_command.contains("RIGHT") {
            player_move = CMD::RIGHT;
        }

        println!("PLAYER_MOVE{:?}", player_move);
        
        let mut players = self.players.lock().await;

        if let Some(player) = players.get_mut(client_id) {
            player.last_command = player_move;
        }
    }

    pub async fn get_game_state(&self, ) -> String {

        let mut players = self.players.lock().await;
        let mut game_state = String::from("{\"Players\":[");

        let mut comma = "";

        for player in players.values_mut() {

            player.movement();

            let player_str = format!(
                "{}{{ \"id\":\"{}\", \"x\": {}, \"y\":{}, \"angle\": {} }}", 
                comma, player.client_id, player.x, player.y, player.angle
            );
            game_state.push_str(&player_str);
            comma = ",";
        }

        game_state.push_str("]}");
        game_state
    }
}
