use crate::collections::bullet_collection::BulletCollection;
use crate::networking::router::{MovePayload};
use crate::types::ClientId;

use crate::collections::asteroid_collection::AsteroidCollection;
use crate::collections::player_collection::PlayerCollection;
use crate::systems::collision::CollisionSystem;

#[derive(Clone)]
pub struct GameManager {
    pub players: PlayerCollection,
    pub asteroids: AsteroidCollection,
    pub bullets: BulletCollection,
}

impl GameManager {
    /// Jogo com RNG por entropia (aleatório a cada execução).
    pub fn new() -> Self {
        Self::build(AsteroidCollection::new(), PlayerCollection::new())
    }

    /// Jogo com seed fixa — RNG reproduzível (para testes).
    pub fn with_seed(seed: u64) -> Self {
        Self::build(
            AsteroidCollection::seeded(seed),
            PlayerCollection::seeded(seed.wrapping_add(1)),
        )
    }

    fn build(mut asteroids: AsteroidCollection, players: PlayerCollection) -> Self {
        for _ in 0..asteroids.max_asteroids {
            asteroids.random_spawn();
        }

        Self {
            players,
            asteroids,
            bullets: BulletCollection::new(),
        }
    }

    pub fn handle_player_command(&mut self, client_id: &ClientId, player_command: &MovePayload) {
        self.players.handle_command(client_id, player_command);
    }

    pub fn collision(&mut self) {
        CollisionSystem::run(
            &mut self.players,
            &mut self.bullets,
            &mut self.asteroids,
        );
    }

    pub fn tick(&mut self, ) {

        let created_bullets = self.players.update();
        self.bullets.add_bullets(created_bullets);
        self.bullets.update();
        self.asteroids.update();
        self.collision();

    }

    pub fn game_info(&self, ) -> String {
        let num_players = self.players.get_players().len();
        let num_bullets = self.bullets.get_bullets().len();
        let num_asteroids = self.asteroids.len();

        format!("Game Info: \n\tPlayers: {} \n\tBullets: {} \n\tAsteroids {}", num_players, num_bullets, num_asteroids)
    }

    fn bullets_to_json(&self, ) -> String {
        // Inicia a construção dos projeteis
        let mut json = String::from("\"Bullets\":[");
        let mut comma = "";

        for bullet in self.bullets.get_bullets().iter() {
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
