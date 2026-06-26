use std::collections::HashMap;
use uuid::Uuid;

use crate::{entities::{
    bullet::Bullet, player::{CMD, Player}
}, networking::router::MovePayload};
use crate::entities::hitbox::HitBox;
use crate::entities::traits::collision_object::CollisionObject;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct PlayerCollection {
    players: HashMap<Uuid, Player>,
    max_players: usize,
    // None = jogo (entropia por entidade); Some = teste/benchmark (reproduzível)
    rng: Option<StdRng>,
}


impl PlayerCollection {
    /// Jogo: sem se preocupar com RNG (entropia).
    pub fn new() -> PlayerCollection {
        Self::build(None)
    }

    /// Teste/benchmark: seed fixa, spawn reproduzível.
    pub fn seeded(seed: u64) -> PlayerCollection {
        Self::build(Some(StdRng::seed_from_u64(seed)))
    }

    fn build(rng: Option<StdRng>) -> PlayerCollection {
        PlayerCollection {
            players: HashMap::new(),
            max_players: 255,
            rng,
        }
    }

    pub fn is_full(&self) -> bool {
        self.players.len() >= self.max_players
    }

    pub fn get_players(&self) -> Vec<Player> {
        self.players.values().cloned().collect()
    }

    pub fn get_hitboxes(&self) -> Vec<HitBox> {
        self.players.values().map(|p| p.hitbox()).collect()
    }

    pub fn get_player(&self, player_id: &Uuid) -> Option<Player> {
        self.players.get(player_id).cloned()
    }

    pub fn get_player_mut(&mut self, id: &Uuid) -> Option<&mut Player> {
        self.players.get_mut(id)
    }

    pub fn add_player(&mut self, client_id: &Uuid) -> Result<Uuid, &'static str> {
        if self.is_full() {
            return Err("Servidor cheio!");
        }

        if self.players.contains_key(client_id) {
            return Err("Player já existe");
        }

        // println!("New player {}", client_id);

        let player = match &mut self.rng {
            Some(rng) => Player::with_rng(client_id, rng),
            None => Player::new(client_id),
        };
        self.players.insert(*client_id, player);

        Ok(*client_id)
    }

    pub fn rm_player(&mut self, client_id: &Uuid) -> bool {
        self.players.remove(client_id).is_some()
    }

    pub fn update(&mut self) -> Vec<Bullet> {
        
        let mut bullets: Vec<Bullet> = Vec::new();

        for player in self.players.values_mut() {

            if let Some(bullet) = player.update() {
                bullets.push(bullet);
            }
        }

        bullets
    }

    // Fora do dominio de player_collection
    pub fn handle_command(&mut self, client_id: &Uuid, player_command: &MovePayload) { 
        if let Some(player) = self.players.get_mut(client_id) { 

            if player_command.thrust    { player.push_command(CMD::UP);     } 
            if player_command.left      { player.push_command(CMD::LEFT);   } 
            if player_command.right     { player.push_command(CMD::RIGHT);  } 
            if player_command.fire      { player.push_command(CMD::SHOT);   } 
        } 
    }

    pub fn to_json(&self,) -> String {
        let mut json = String::from("\"Players\":[");
        let mut comma = "";

        for player in self.players.values() {
            let player_str = format!("{}{}", comma, player.to_json());
            json.push_str(&player_str);
            comma = ",";
        }

        json.push(']');
        json
    }
}

