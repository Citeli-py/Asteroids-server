use crate::collision_object::CollisionObject;
use crate::types::{ClientId};
use crate::player::{Player, CMD};
use crate::bullet_collection::*;

use std::collections::HashMap;

#[derive(Clone)]
pub struct GameManager {
    players: HashMap<ClientId, Player>, // Separar colection em outra classe
    bullets: BulletCollection,
}

impl GameManager {
    pub fn new() -> Self {
        let players = HashMap::new();
        Self { players, bullets: BulletCollection::new() }
    }

    pub fn add_player(&mut self, client_id: &ClientId) {
        println!("New player");
        let new_player = Player::new(client_id);

        self.players.insert(*client_id, new_player);
    }

    pub fn rm_player(&mut self, client_id: &ClientId) {
        self.players.remove(client_id);
    }

    pub fn handle_player_command(&mut self, client_id: &ClientId, player_command: &String) {
        /*
        UP
        LEFT
        RIGHT
        SHOT
        */

        if let Some(player) = self.players.get_mut(client_id) {

            if player_command.contains("UP") {
                player.push_command(CMD::UP);
            }
    
            if player_command.contains("LEFT") {
                player.push_command(CMD::LEFT);
            }
    
            if player_command.contains("RIGHT") {
                player.push_command(CMD::RIGHT);
            }

            if player_command.contains("SHOT") {
                player.push_command(CMD::SHOT);
            }

        }
        
    }

    pub fn get_game_state(&mut self, ) -> String {

        let mut game_state = String::from("{\"Players\":[");

        let mut comma = "";

        for player in self.players.values_mut() {

            if let Some(new_bullet) = player.update() {
                self.bullets.add_bullet(new_bullet);
            }

            let player_str = format!("{} {}", comma, player.to_json());
            game_state.push_str(&player_str);
            comma = ",";
        }

        // Fecha a informação dos players
        game_state.push_str("],");

        self.bullets.update();
        // Inicia a construção dos projeteis
        game_state.push_str(&self.bullets.to_json());

        // cria uma lista de players (só referências imutáveis pra checar colisão)
        let bullets: Vec<_> = self.bullets.bullets.clone(); // se Bullet: Clone
        let players: Vec<_> = self.players.values().cloned().collect();

        for player in &players {
            for bullet in &bullets {
                if bullet.player_id != player.get_id() && bullet.has_collision(player) {
                    self.bullets.rm_bullet(bullet.id);
                    self.rm_player(&player.get_id());
                }
            }
        }

        game_state.push_str("}");
        game_state
    }
}
