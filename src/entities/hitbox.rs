use uuid::Uuid;

// Collision layers (1 bit cada). Com 3 tipos de entidade, u8 sobra.
pub const LAYER_PLAYER: u8 = 1 << 0;
pub const LAYER_BULLET: u8 = 1 << 1;
pub const LAYER_ASTEROID: u8 = 1 << 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Bullet,
    Asteroid,
}

impl EntityKind {

    pub fn layer(&self) -> u8 {
        match self {
            EntityKind::Player => LAYER_PLAYER,
            EntityKind::Bullet => LAYER_BULLET,
            EntityKind::Asteroid => LAYER_ASTEROID,
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            EntityKind::Bullet => 0,
            EntityKind::Player => 1,
            EntityKind::Asteroid => 2,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Geometria relativa ao `pos` do HitBox (sem posição absoluta).
#[derive(Clone, Copy)]
pub enum Shape {
    Circle { radius: f32 },
}

#[derive(Clone, Copy)]
pub struct HitBox {
    pub id: Uuid,         // id da entidade dona, pra remoção/resolução
    pub kind: EntityKind, // tipo da entidade (layer derivado dele)
    pub pos: Point,       // posição absoluta, fonte única
    pub shape: Shape,     // geometria relativa ao pos
    pub mask: u8,         // com quais layers essa entidade colide
}

impl HitBox {
    pub fn circle(id: Uuid, kind: EntityKind, pos: (f32, f32), radius: f32, mask: u8) -> HitBox {
        HitBox {
            id,
            kind,
            pos: Point { x: pos.0, y: pos.1 },
            shape: Shape::Circle { radius },
            mask,
        }
    }

    /// Só testa colisão se ambos os lados se importam com o layer do outro.
    pub fn should_collide(&self, other: &HitBox) -> bool {
        (self.mask & other.kind.layer() != 0) && (other.mask & self.kind.layer() != 0)
    }

    /// Despacha a colisão por par de formas. Por enquanto só círculo×círculo.
    pub fn intersects(&self, other: &HitBox) -> bool {
        match (self.shape, other.shape) {
            (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
                HitBox::circle_circle_colision(
                    (self.pos, r1), 
                    (other.pos, r2)
                )
            }

            _ => false
        }
    }

    pub fn ord<'a>(x: &'a HitBox, y: &'a HitBox) -> (&'a HitBox, &'a HitBox) {
        if x.kind.rank() <= y.kind.rank() { 
            return (x, y) 
        } 

        (y, x)    
    }

    fn circle_circle_colision(c1: (Point, f32), c2: (Point, f32)) -> bool {
        let dx = c1.0.x - c2.0.x;
        let dy = c1.0.y - c2.0.y;
        let dist_sq = dx * dx + dy * dy;
        let r = c1.1 + c2.1;
        dist_sq <= r * r
    }
}
