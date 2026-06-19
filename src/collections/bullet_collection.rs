use uuid::Uuid;
use crate::entities::bullet::Bullet;

const MAX_BULLETS: usize = 2048;

#[derive(Clone)]
pub struct BulletCollection {
    bullets: Vec<Bullet>,
    max_bullets: usize,
}

impl BulletCollection {
    pub fn new() -> BulletCollection {
        BulletCollection { 
            bullets: Vec::with_capacity(MAX_BULLETS), 
            max_bullets: MAX_BULLETS 
        }
    }

    pub fn get_bullets(&self, ) -> Vec<Bullet> {
        self.bullets.clone()
    }

    pub fn add_bullet(&mut self, bullet: Bullet) -> bool {
        if self.bullets.len() >= self.max_bullets {
            return false;
        }
        self.bullets.push(bullet);
        true
    }

    pub fn add_bullets(&mut self, bullets: Vec<Bullet>) -> bool {
        let mut max_push = bullets.len();
        
        if self.max_bullets <= self.bullets.len()+bullets.len() { // if overflow
            max_push = self.max_bullets-self.bullets.len();
        };

        self.bullets.extend(bullets.into_iter().take(max_push));
        true
    }

    pub fn rm_bullet(&mut self, bullet_id: Uuid) -> bool {
        let before = self.bullets.len();
        self.bullets.retain(|b| b.id != bullet_id);
        before != self.bullets.len() // true se removeu algo
    }

    pub fn update(&mut self) {

        self.bullets.retain_mut(|bullet| {
            bullet.update();
            !bullet.is_destroyed()
        });
    }


    pub fn to_json(&self) -> String {
        let mut json = String::from("\"Bullets\":[");
        let mut comma = "";

        for bullet in self.bullets.iter() {
            let bullet_str = format!("{} {}", comma, &bullet.to_json());
            json.push_str(&bullet_str);
            comma = ",";
        }

        json += "]";
        return json;
    }
}
