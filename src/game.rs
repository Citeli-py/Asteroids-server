use crate::{asteroid, bullet, player};
use crate::collision_object::CollisionObject;
use crate::types::{ClientId, WORLD_SIZE,};
use crate::player::{Player, CMD};
use uuid::Uuid;
use crate::bullet::Bullet;

use rand::Rng;

use crate::asteroid_collection::AsteroidCollection;
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
    pub asteroids: AsteroidCollection
}

impl GameManager {
    pub fn new() -> Self {
        let mut asteroids = AsteroidCollection::new();
        let mut rng = rand::rng();

        for i in 0..5 {

            let x = rng.random_range(0.0..(WORLD_SIZE as f32));
            let y = rng.random_range(0.0..(WORLD_SIZE as f32));

            asteroids.spawn(x, y, asteroid::AsteroidType::BIG);
        }
        

        Self { 
            players: PlayerCollection::new(),
            asteroids: asteroids //AsteroidCollection::new()
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

        for player in &players_snapshot {
            for bullet in &bullets_snapshot {

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

        let mut hitted_asteroids: Vec<(usize, Uuid, Option<Uuid>)> = Vec::new();
        for player in &players_snapshot {
            for (i, asteroid) in self.asteroids.get_all().iter().enumerate(){
                if player.has_collision(asteroid) {
                    hitted_asteroids.push((i, player.get_id(), None));
                }
            }
        }

        for bullet in &bullets_snapshot {
            for (i, asteroid) in self.asteroids.get_all().iter().enumerate(){
                if bullet.has_collision(asteroid) {
                    hitted_asteroids.push((i, bullet.player_id, Some(bullet.id)));
                }
            }
        }

        for hit  in hitted_asteroids.iter().rev() {
            
            let (asteroid_id, player_id, bullet) = hit;
            self.asteroids.remove_at(*asteroid_id);

            match bullet {
                Some(bullet_id) => self.players.get_player_mut(player_id).unwrap().bullets.rm_bullet(*bullet_id),
                None => self.players.rm_player(player_id)
            };
        }
        

    }

    pub fn tick(&mut self, ) {

        self.players.update();
        self.asteroids.update();
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
        game_state.push_str(",");

        // Asteroids para json
        game_state.push_str(&self.asteroids.to_json());
        game_state.push_str("}");
        game_state
    }
}
