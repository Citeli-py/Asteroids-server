use uuid::Uuid;

use crate::collections::asteroid_collection::AsteroidCollection;
use crate::collections::bullet_collection::BulletCollection;
use crate::collections::player_collection::PlayerCollection;
use crate::entities::hitbox::{EntityKind, HitBox};

struct Hit {
    a: (EntityKind, Uuid),
    b: (EntityKind, Uuid)
}

impl Hit {
    fn from_pair(x: &HitBox, y: &HitBox) -> Option<Hit> {

        let (a, b) = HitBox::ord(x, y);

        match (a.kind, b.kind) {
            (EntityKind::Bullet, EntityKind::Player) => {
                Some(Hit {
                    a: (EntityKind::Bullet, a.id),
                    b: (EntityKind::Player, b.id)
                })
            }
            (EntityKind::Bullet, EntityKind::Asteroid) => Some(Hit {
                a: (EntityKind::Bullet, a.id),
                b: (EntityKind::Asteroid, b.id)
            }),
            (EntityKind::Player, EntityKind::Asteroid) => Some(Hit {
                a: (EntityKind::Player, a.id),
                b: (EntityKind::Asteroid, b.id)
            }),

            _ => None,
        }
    }
}

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn run(
        players: &mut PlayerCollection,
        bullets: &mut BulletCollection,
        asteroids: &mut AsteroidCollection,
    ) {

        let mut boxes = players.get_hitboxes();
        boxes.extend(bullets.get_hitboxes());
        boxes.extend(asteroids.get_hitboxes());

        let hits = CollisionSystem::colide(&boxes);

        for hit in hits {
            Self::resolve(players, bullets, asteroids, &hit);
        }
    }


    fn colide(boxes: &Vec<HitBox>) -> Vec<Hit> {
        let mut hits: Vec<Hit> = Vec::new();

        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                let a = &boxes[i];
                let b = &boxes[j];

                if !( a.should_collide(b) && a.intersects(b) ) {
                    continue;
                }

                if let Some(hit) = Hit::from_pair(a, b) {
                    hits.push(hit);
                }
            }
        }
        hits
    }


    fn resolve(
        players: &mut PlayerCollection,
        bullets: &mut BulletCollection,
        asteroids: &mut AsteroidCollection,
        hit: &Hit,
    ) {
        let (a_kind, a_id) = hit.a;
        let (b_kind, b_id) = hit.b;

        if let (EntityKind::Bullet, EntityKind::Player) = (a_kind, b_kind) {
            if bullets.get_owner(&a_id) == Some(b_id) {
                return;
            }
        }

        Self::remove_entity(players, bullets, asteroids, a_kind, a_id);
        Self::remove_entity(players, bullets, asteroids, b_kind, b_id);
    }

    fn remove_entity(
        players: &mut PlayerCollection,
        bullets: &mut BulletCollection,
        asteroids: &mut AsteroidCollection,
        kind: EntityKind,
        id: Uuid,
    ) {
        match kind {
            EntityKind::Player      => { players.rm_player(&id); }
            EntityKind::Asteroid    => { asteroids.remove_by_id(id); }
            EntityKind::Bullet      => { bullets.rm_bullet(id); }
        }
    }
}
