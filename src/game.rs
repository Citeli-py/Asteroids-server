use crate::collision_object::CollisionObject;
use crate::types::{ClientId};
use crate::player::{Player, CMD};

use crate::bullet_collection::BulletCollection;
use crate::player_collection::PlayerCollection;

#[derive(Clone)]
pub struct GameManager {
    pub players: PlayerCollection,
}

impl GameManager {
    pub fn new() -> Self {
        Self { 
            players: PlayerCollection::new()
        }
    }

    pub fn handle_player_command(&mut self, client_id: &ClientId, player_command: &String) {
        self.players.handle_command(client_id, player_command);
    }

    fn bullets_to_json(&self, ) -> String {
        // Inicia a construção dos projeteis
        let mut json = String::from("\"Bullets\":[");
        let mut comma = "";

        for bullet in self.players.get_all_bullets().iter() {
            let bullet_str = format!("{} {}", comma, &bullet.to_json());
            json.push_str(&bullet_str);
            comma = ",";
        }

        json += "]";
        return json;
    }

    pub fn get_game_state(&mut self, ) -> String {

        let mut game_state = String::from("{");

        self.players.update();
        game_state.push_str(&self.players.to_json());
        game_state.push_str(",");

        // Inicia a construção dos projeteis
        game_state.push_str(&self.bullets_to_json());
        game_state.push_str("}");
        game_state
    }
}
