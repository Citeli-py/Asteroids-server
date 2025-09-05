use crate::types::{ClientId, TICK_RATE};
use crate::collision_object::CollisionObject;

pub struct Bullet {
    pub player_id: ClientId,
    pub x: f32,
    pub y: f32,
    v: f32,
    angle: f32
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
            player_id: player_id,
            x: x0, 
            y: y0, 
            angle: angle,
            v: 10.0
        }
    }

    pub fn update(&mut self) {
        let dt = (1/TICK_RATE) as f32;
        self.x += f32::cos(self.angle)*dt;
        self.y += f32::sin(self.angle)*dt;
    }

    pub fn get_position(&self, ) -> String {
        format!("{{ \"x\": {}, \"y\": {} }}", self.x, self.y)
    }
}