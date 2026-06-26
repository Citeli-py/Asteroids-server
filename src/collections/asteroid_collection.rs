use std::collections::HashMap;
use uuid::Uuid;

use crate::entities::asteroid::{Asteroid, AsteroidType};
use crate::types::{TICK_RATE, WORLD_SIZE};
use crate::entities::traits::warp_object::WarpObject;
use crate::entities::hitbox::HitBox;
use crate::entities::traits::collision_object::CollisionObject;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct AsteroidCollection {
    asteroids:      HashMap<Uuid, Asteroid>,
    pub max_asteroids:  usize,

    spawn_counter:  u32,
    spawn_cooldown: u32,
    // None = jogo (entropia por entidade); Some = teste/benchmark (reproduzível)
    rng: Option<StdRng>,
}

impl AsteroidCollection {
    /// Jogo: sem se preocupar com RNG (entropia).
    pub fn new() -> Self {
        Self::build(None)
    }

    /// Teste/benchmark: seed fixa, posições reproduzíveis.
    pub fn seeded(seed: u64) -> Self {
        Self::build(Some(StdRng::seed_from_u64(seed)))
    }

    fn build(rng: Option<StdRng>) -> Self {
        let tick = TICK_RATE as f32;

        Self {
            asteroids: HashMap::new(),
            max_asteroids: 16,

            spawn_cooldown: (60.0 * tick) as u32,
            spawn_counter: (1.0 * tick) as u32,
            rng,
        }
    }

    pub fn spawn( &mut self, x: f32, y: f32, tier: AsteroidType) -> bool {
        if tier == AsteroidType::BIG && self.asteroids.len() >= self.max_asteroids {
            return false;
        }

        let asteroid = match &mut self.rng {
            Some(rng) => Asteroid::with_rng(x, y, tier, rng),
            None => Asteroid::new(x, y, tier),
        };
        self.asteroids.insert(asteroid.id, asteroid);
        true
    }

    pub fn random_spawn(&mut self, ) -> bool {
        let (x, y) = match &mut self.rng {
            Some(rng) => (
                rng.random_range(0.0..(WORLD_SIZE as f32)),
                rng.random_range(0.0..(WORLD_SIZE as f32)),
            ),
            None => (
                rand::random_range(0.0..(WORLD_SIZE as f32)),
                rand::random_range(0.0..(WORLD_SIZE as f32)),
            ),
        };
        self.spawn(x, y, AsteroidType::BIG)
    }

    pub fn update(&mut self) {
        for asteroid in self.asteroids.values_mut() {
            asteroid.update();
            asteroid.warp();
        }

        if self.spawn_counter >= self.spawn_cooldown {
            println!("Asteroid spawn");
            self.random_spawn();
            self.spawn_counter = 0;
        }

        self.spawn_counter += 1;
    }

    pub fn remove_by_id(&mut self, id: Uuid) -> bool {
        match self.asteroids.remove(&id) {
            Some(asteroid) => {
                self.split(asteroid);
                true
            }
            None => false,
        }
    }

    /// Ao destruir um asteroide, gera os filhos do tier menor.
    fn split(&mut self, asteroid: Asteroid) {
        let child_size = match asteroid.size {
            AsteroidType::BIG => Some(AsteroidType::MEDIUM),
            AsteroidType::MEDIUM => Some(AsteroidType::SMALL),
            AsteroidType::SMALL => None,
        };

        if let Some(child_size) = child_size {
            self.spawn(asteroid.x, asteroid.y, child_size);
            self.spawn(asteroid.x, asteroid.y, child_size);
        }
    }

    pub fn clear(&mut self) {
        self.asteroids.clear();
    }

    pub fn get_hitboxes(&self) -> Vec<HitBox> {
        self.asteroids.values().map(|a| a.hitbox()).collect()
    }

    pub fn len(&self) -> usize {
        self.asteroids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.asteroids.is_empty()
    }

    pub fn to_json(&self,) -> String {
        let mut json = String::from("\"Asteroids\":[");
        let mut comma = "";

        for asteroid in self.asteroids.values() {
            let asteroid_str = format!("{} {}", comma, &asteroid.to_json());
            json.push_str(&asteroid_str);
            comma = ",";
        }

        json += "]";
        return json;
    }
}
