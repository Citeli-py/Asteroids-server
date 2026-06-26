use std::collections::HashMap;
use uuid::Uuid;
use crate::entities::bullet::Bullet;
use crate::entities::hitbox::HitBox;
use crate::entities::traits::collision_object::CollisionObject;

pub const MAX_BULLETS: usize = 2048;

#[derive(Clone)]
pub struct BulletCollection {
    bullets: HashMap<Uuid, Bullet>,
    max_bullets: usize,
}

impl BulletCollection {
    pub fn new() -> BulletCollection {
        BulletCollection {
            bullets: HashMap::with_capacity(MAX_BULLETS),
            max_bullets: MAX_BULLETS,
        }
    }

    pub fn get_bullets(&self, ) -> Vec<Bullet> {
        self.bullets.values().cloned().collect()
    }

    pub fn get_hitboxes(&self) -> Vec<HitBox> {
        self.bullets.values().map(|b| b.hitbox()).collect()
    }

    pub fn add_bullet(&mut self, bullet: Bullet) -> bool {
        if self.bullets.len() >= self.max_bullets {
            return false;
        }
        self.bullets.insert(bullet.id, bullet);
        true
    }

    pub fn add_bullets(&mut self, bullets: Vec<Bullet>) -> bool {
        // só insere até o que cabe no limite
        let free = self.max_bullets.saturating_sub(self.bullets.len());

        for bullet in bullets.into_iter().take(free) {
            self.bullets.insert(bullet.id, bullet);
        }
        true
    }

    pub fn rm_bullet(&mut self, bullet_id: Uuid) -> bool {
        self.bullets.remove(&bullet_id).is_some()
    }

    /// Quem atirou essa bala (pra atribuir pontos na resolução do hit).
    pub fn get_owner(&self, bullet_id: &Uuid) -> Option<Uuid> {
        self.bullets.get(bullet_id).map(|b| b.player_id)
    }

    pub fn update(&mut self) {
        self.bullets.retain(|_, bullet| {
            bullet.update();
            !bullet.is_destroyed()
        });
    }


    pub fn to_json(&self) -> String {
        let mut json = String::from("\"Bullets\":[");
        let mut comma = "";

        for bullet in self.bullets.values() {
            let bullet_str = format!("{} {}", comma, &bullet.to_json());
            json.push_str(&bullet_str);
            comma = ",";
        }

        json += "]";
        return json;
    }
}
