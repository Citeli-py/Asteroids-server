use uuid::Uuid;

use crate::collision_object::CollisionObject;
use crate::warp_object::WarpObject;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AsteroidType {
    SMALL,
    MEDIUM,
    BIG
}

#[derive(Clone)]
pub struct Asteroid {
    pub id: Uuid,
    pub x: f32,
    pub y: f32,
    pub radius: u8,
    pub size: AsteroidType,
    v: f32,
    angle: f32
}


impl CollisionObject for Asteroid {
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn radius(&self) -> f32 {
        self.radius as f32
    }
}

impl WarpObject for Asteroid {
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Asteroid {

    pub fn new(x: f32, y: f32, size: AsteroidType) -> Asteroid {
        let r = match size {
            AsteroidType::BIG => 35,
            AsteroidType::MEDIUM => 25,
            AsteroidType::SMALL => 15,
        };

        let v = match size {
            AsteroidType::BIG => 2.0,
            AsteroidType::MEDIUM => 4.0,
            AsteroidType::SMALL => 6.0,
        };

        Asteroid {id: Uuid::new_v4(), x, y, radius: r, v, size: size, angle: rand::random_range(0.0..6.28) }
    }

    pub fn update(&mut self) {
        self.x += self.v*f32::cos(self.angle);
        self.y += self.v*f32::sin(self.angle);
        
        (self.x, self.y) = self.warp();
    }

    pub fn to_json(&self, ) -> String {
        format!("{{\"id\": \"{}\", \"radius\": \"{}\", \"x\": {}, \"y\": {} }}", self.id, self.radius, self.x, self.y)
    }
}