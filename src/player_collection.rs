use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    bullet::Bullet,
    collision_object::CollisionObject,
    player::{self, CMD, Player},
};

struct Hit {
    player_id: Uuid,   // quem atirou
    hitted_id: Uuid,   // quem foi atingido
    bullet_id: Uuid,
}

impl Hit {
    pub fn new(player: &Player, bullet: &Bullet) -> Hit {
        Hit {
            player_id: bullet.player_id,
            hitted_id: player.get_id(),
            bullet_id: bullet.id,
        }
    }
}

#[derive(Clone)]
pub struct PlayerCollection {
    players: HashMap<Uuid, Player>,
    max_players: usize,
}


impl PlayerCollection {
    pub fn new() -> PlayerCollection {
        PlayerCollection {
            players: HashMap::new(),
            max_players: 255,
        }
    }

    pub fn is_full(&self) -> bool {
        self.players.len() >= self.max_players
    }

    pub fn get_players(&self) -> Vec<Player> {
        self.players.values().cloned().collect()
    }

    pub fn get_player(&self, player_id: &Uuid) -> Option<Player> {
        self.players.get(player_id).cloned()
    }


    // Fora do dominio de PlayerCollection
    pub fn get_all_bullets(&self, ) -> Vec<Bullet> {
        let mut all_bullets: Vec<Bullet> = Vec::new();
        for player in self.get_players() {
            all_bullets.extend(player.bullets.get_bullets());
        }

        return all_bullets;
    }

    pub fn add_player(&mut self, client_id: &Uuid) -> Result<Uuid, &'static str> {
        if self.is_full() {
            return Err("Servidor cheio!");
        }

        if self.players.contains_key(client_id) {
            return Err("Player já existe");
        }

        println!("New player {}", client_id);

        let player = Player::new(client_id);
        self.players.insert(*client_id, player);

        Ok(*client_id)
    }

    pub fn rm_player(&mut self, client_id: &Uuid) -> bool {
        self.players.remove(client_id).is_some()
    }

    pub fn update(&mut self) {
        for player in self.players.values_mut() {
            player.update();
            player.bullets.update();
        }

        self.collision();
    }

    pub fn collision(&mut self) {
        let mut hits: Vec<Hit> = Vec::new();

        for bullet in self.get_all_bullets() {
            for player in self.get_players() {

                if player.get_id() == bullet.player_id {
                    continue;
                }

                if player.has_collision(&bullet) {
                    hits.push(Hit::new(&player, &bullet));
                }
                
            }
        }

        // aplicar efeitos
        for hit in hits {
            // remove player atingido
            self.rm_player(&hit.hitted_id);

            // remove bala do atirador
            if let Some(shooter) = self.players.get_mut(&hit.player_id) {
                shooter.bullets.rm_bullet(hit.bullet_id);
            }
        }
    }

    // Fora do dominio de player_collection
    pub fn handle_command(&mut self, client_id: &Uuid, player_command: &String) { 
        if let Some(player) = self.players.get_mut(client_id) { 

            if player_command.contains("UP")    { player.push_command(CMD::UP);     } 
            if player_command.contains("LEFT")  { player.push_command(CMD::LEFT);   } 
            if player_command.contains("RIGHT") { player.push_command(CMD::RIGHT);  } 
            if player_command.contains("SHOT")  { player.push_command(CMD::SHOT);   } 
        } 
    }

    pub fn to_json(&self) -> String {
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

