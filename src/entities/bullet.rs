use uuid::Uuid;

use crate::types::{ClientId, TICK_RATE};
use crate::entities::traits::collision_object::CollisionObject;
use crate::entities::traits::warp_object::WarpObject;
use crate::entities::hitbox::{HitBox, EntityKind, LAYER_PLAYER, LAYER_ASTEROID};

#[derive(Clone)]
pub struct Bullet {
    pub id: Uuid,
    pub player_id: ClientId,
    pub x: f32,
    pub y: f32,
    pub v: f32,
    pub angle: f32,
    ttl: u32
}


impl CollisionObject for Bullet {
    
    fn hitbox(&self) -> HitBox {
        HitBox::circle(
            self.id,
            EntityKind::Bullet,
            (self.x, self.y),
            5.0,
            LAYER_PLAYER | LAYER_ASTEROID,
        )
    }
}

impl WarpObject for Bullet {
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}


impl Bullet {

    pub fn new(player_id: ClientId, x0: f32, y0: f32, v0: f32, angle: f32) -> Bullet {
        Bullet{ 
            id: Uuid::new_v4(),
            player_id: player_id,
            x: x0, 
            y: y0, 
            angle: angle,
            v: v0 + (450.0 / TICK_RATE as f32),
            ttl: 5*TICK_RATE as u32,
        }
    }

    pub fn update(&mut self) {
        self.x += self.v*f32::cos(self.angle);
        self.y += self.v*f32::sin(self.angle);

        // Warp
        (self.x, self.y) = self.warp();

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
        format!("{{\"id\": \"{}\", \"player_id\": \"{}\", \"x\": {}, \"y\": {}, \"angle\": {} }}", self.id, self.player_id, self.x, self.y, self.angle)
    }
}