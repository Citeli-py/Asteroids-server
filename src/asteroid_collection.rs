use crate::asteroid::{self, Asteroid, AsteroidType};
use crate::types::{TICK_RATE, WORLD_SIZE};
use crate::warp_object::WarpObject;

#[derive(Clone)]
pub struct AsteroidCollection {
    asteroids:      Vec<Asteroid>,
    pub max_asteroids:  usize,

    spawn_counter:  u32,
    spawn_cooldown: u32,
}

impl AsteroidCollection {
    pub fn new() -> Self {

        let tick = TICK_RATE as f32;

        Self {
            asteroids: Vec::new(),
            max_asteroids: 16,

            spawn_cooldown: (60.0 * tick) as u32,
            spawn_counter: (1.0 * tick) as u32,
        }
    }

    pub fn spawn( &mut self, x: f32, y: f32, tier: AsteroidType) -> bool {
        if tier == AsteroidType::BIG && self.asteroids.len() >= self.max_asteroids {
            return false;
        }

        self.asteroids.push(Asteroid::new(x, y, tier));
        true
    }

    pub fn random_spawn(&mut self, ) -> bool {
        self.spawn(
             rand::random_range(0.0..(WORLD_SIZE as f32)), 
             rand::random_range(0.0..(WORLD_SIZE as f32)),  
             AsteroidType::BIG)
    }

    pub fn update(&mut self) {
        for asteroid in self.asteroids.iter_mut() {
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

    pub fn remove_at(&mut self, index: usize) {

        if index >= self.asteroids.len() {
            return;
        }

        let asteroid = self.asteroids.swap_remove(index);

        let option_child_size = match asteroid.size {
            AsteroidType::BIG => Some(AsteroidType::MEDIUM),
            AsteroidType::MEDIUM => Some(AsteroidType::SMALL),
            AsteroidType::SMALL => None
        };

        if let Some(child_size) = option_child_size {
            self.spawn(asteroid.x, asteroid.y, child_size);
            self.spawn(asteroid.x, asteroid.y, child_size);
        }

    }

    pub fn clear(&mut self) {
        self.asteroids.clear();
    }

    pub fn get_all(&self) -> &Vec<Asteroid> {
        &self.asteroids
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

        for asteroid in self.asteroids.iter() {
            let asteroid_str = format!("{} {}", comma, &asteroid.to_json());
            json.push_str(&asteroid_str);
            comma = ",";
        }

        json += "]";
        return json;
    }
}
