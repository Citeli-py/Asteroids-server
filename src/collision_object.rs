

pub trait CollisionObject {
    
    fn position(&self) -> (f32, f32);
    fn radius(&self) -> f32;

    fn has_collision<T: CollisionObject>(&self, other: &T) -> bool {
        let (x1, y1) = self.position();
        let (x2, y2) = other.position();

        let dx = x1 - x2;
        let dy = y1 - y2;
        let dist_sq = dx * dx + dy * dy;
        let r = self.radius() + other.radius();

        dist_sq <= r * r
    }
}
