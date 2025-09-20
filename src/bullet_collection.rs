use uuid::Uuid;
use crate::bullet::Bullet;

#[derive(Clone)]
pub struct BulletCollection {
    bullets: Vec<Bullet>,
    max_bullets: usize,
}

impl BulletCollection {
    pub fn new() -> BulletCollection {
        BulletCollection { 
            bullets: Vec::new(), 
            max_bullets: 255 
        }
    }

    pub fn add_bullet(&mut self, bullet: Bullet) -> bool {
        if self.bullets.len() >= self.max_bullets {
            return false;
        }
        self.bullets.push(bullet);
        true
    }

    pub fn rm_bullet(&mut self, bullet_id: Uuid) -> bool {
        let before = self.bullets.len();
        self.bullets.retain(|b| b.id != bullet_id);
        before != self.bullets.len() // true se removeu algo
    }

    pub fn update(&mut self) {
        // atualiza cada bullet
        for bullet in &mut self.bullets {
            bullet.update();
        }

    }

    pub fn to_json(&self) -> String {
        let mut json = String::from("\"Bullets\":[");

        for bullet in self.bullets.iter() {
            json.push_str(&bullet.to_json());
            json += ",";
        }

        //json.pop();

        json += "]";
        return json;
    }
}
