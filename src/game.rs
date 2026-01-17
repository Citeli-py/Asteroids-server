use crate::collision_object::CollisionObject;
use crate::types::{ClientId,};
use crate::player::{Player, CMD};
use uuid::Uuid;
use crate::bullet::Bullet;

use crate::bullet_collection::BulletCollection;
use crate::player_collection::PlayerCollection;

struct Hit {
    shooter_id: Uuid,   // quem atirou
    target_id: Uuid,   // quem foi atingido
    bullet_id: Uuid,
}

impl Hit {
    pub fn new(player: &Player, bullet: &Bullet) -> Hit {
        Hit {
            shooter_id: bullet.player_id,
            target_id: player.get_id(),
            bullet_id: bullet.id,
        }
    }
}

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

    pub fn collision(&mut self) {
        let mut hits: Vec<Hit> = Vec::new();

        // ---------- FASE 1: DETECÇÃO ----------
        let players_snapshot = self.players.get_players();
        let bullets_snapshot = self.players.get_all_bullets();

        for bullet in &bullets_snapshot {
            for player in &players_snapshot {

                // não colide com o próprio atirador
                if player.get_id() == bullet.player_id {
                    continue;
                }

                if player.has_collision(bullet) {
                    hits.push(Hit {
                        shooter_id: bullet.player_id,
                        target_id: player.get_id(),
                        bullet_id: bullet.id,
                    });
                }
            }
        }

        // ---------- FASE 2: APLICAÇÃO ----------
        for hit in hits {
            // remove o player atingido
            self.players.rm_player(&hit.target_id);

            // remove a bala do atirador REAL
            if let Some(shooter) = self.players.get_player_mut(&hit.shooter_id) {
                shooter.bullets.rm_bullet(hit.bullet_id);
            }
        }
    }

    pub fn tick(&mut self, ) {
        self.players.update();
        self.collision();
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

        game_state.push_str(&self.players.to_json());
        game_state.push_str(",");

        // Inicia a construção dos projeteis
        game_state.push_str(&self.bullets_to_json());
        game_state.push_str("}");
        game_state
    }
}
