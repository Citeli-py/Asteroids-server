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
    input_buffer: Vec<CMD>,
    buffer_size: usize,
    client_id: ClientId,
}

impl Player {
    pub fn new(client_id: &ClientId) -> Player {
        Player {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            turn_speed: 0.2,
            acceleration: 0.2,
            friction: 0.999,
            input_buffer: vec![],
            buffer_size: 2,
            client_id: client_id.clone(),
        }
    }

    fn push_command(&mut self, cmd: CMD) {
        if self.input_buffer.len() >= self.buffer_size {
            self.input_buffer.remove(0); // Remove o comando mais antigo
        }
        self.input_buffer.push(cmd);
    }

    pub fn clear_input_buffer(&mut self, ) {
        self.input_buffer = vec![CMD::NONE];
    }

    pub fn update(&mut self) {
        let dt = 1.0 / TICK_RATE as f32;

        (self.x, self.y, self.vx, self.vy, self.angle) = self.apply_commands(self.input_buffer.clone(), dt);

        self.clear_input_buffer();
    }

    fn apply_commands(&self, commands: Vec<CMD>, dt: f32) -> (f32, f32, f32, f32, f32) {
        let (mut x, mut y, mut vx, mut vy, mut angle) =
            (self.x, self.y, self.vx, self.vy, self.angle);

        // Processa cada comando
        for cmd in commands.into_iter() {
            match cmd {
                CMD::UP => {
                    vx += self.acceleration * f32::cos(angle) * dt;
                    vy += self.acceleration * f32::sin(angle) * dt;
                }
                CMD::LEFT => {
                    angle -= self.turn_speed * dt;
                }
                CMD::RIGHT => {
                    angle += self.turn_speed * dt;
                }
                CMD::SHOT => {
                    // Tiro
                }
                CMD::NONE => {
                    // Nada - apenas aplica a física básica
                }
            }

            // Aplica física mesmo sem comandos
            x += vx * dt;
            y += vy * dt;
            
            // Aplica fricção
            vx *= self.friction;
            vy *= self.friction;
        }

        (x, y, vx, vy, angle)
    }


    pub fn get_position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.angle)
    }

    pub fn get_id(&self) -> ClientId {
        self.client_id.clone()
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
