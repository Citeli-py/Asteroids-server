use uuid::Uuid;

use crate::types::{ClientId, TICK_RATE};
use crate::collision_object::CollisionObject;

#[derive(Clone)]
pub struct Bullet {
    pub id: Uuid,
    pub player_id: ClientId,
    pub x: f32,
    pub y: f32,
    v: f32,
    angle: f32,
    ttl: u32
}


impl CollisionObject for Bullet {
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn radius(&self) -> f32 {
        2.0
    }
}


impl Bullet {

    pub fn new(player_id: ClientId, x0: f32, y0: f32, angle: f32) -> Bullet {
        Bullet{ 
            id: Uuid::new_v4(),
            player_id: player_id,
            x: x0, 
            y: y0, 
            angle: angle,
            v: 3.0,
            ttl: 8192
        }
    }

    pub fn update(&mut self) {
        let dt = 1.0 / TICK_RATE as f32;
        self.x += self.v*f32::cos(self.angle)*dt;
        self.y += self.v*f32::sin(self.angle)*dt;

        if self.ttl > 0 {
            self.ttl -= 1;
        }
    }

    pub fn is_destroyed(&self, ) -> bool {
        self.ttl == 0
    }

    pub fn destroy(&mut self, ) {
        self.ttl=0;
    }

    pub fn to_json(&self, ) -> String {
        format!("{{\"id\": \"{}\", \"player_id\": \"{}\", \"x\": {}, \"y\": {} }}", self.id, self.player_id, self.x, self.y)
    }
}