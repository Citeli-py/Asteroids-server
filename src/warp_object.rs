use crate::types::WORLD_SIZE;


pub trait WarpObject {

    fn position(&self) -> (f32, f32);
    
    fn warp(&self) -> (f32, f32) {
        let (x, y) = self.position();

        let warp_x = if x < 0.0 { WORLD_SIZE as f32 } else { x%(WORLD_SIZE as f32) };
        let warp_y = if y < 0.0 { WORLD_SIZE as f32 } else { y%(WORLD_SIZE as f32) };

        return (warp_x, warp_y);
    }
}
