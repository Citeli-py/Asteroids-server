use crate::entities::hitbox::HitBox;

pub trait CollisionObject {

    /// Cada entidade monta seu HitBox por tick a partir do estado atual.
    fn hitbox(&self) -> HitBox;

    fn has_collision<T: CollisionObject>(&self, other: &T) -> bool {
        let a = self.hitbox();
        let b = other.hitbox();

        a.should_collide(&b) && a.intersects(&b)
    }
}
