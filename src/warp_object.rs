use crate::types::WORLD_SIZE;

pub trait WarpObject {

    fn position(&self) -> (f32, f32);
    
    fn warp(&self) -> (f32, f32) {
        let (x, y) = self.position();
        let warp_x = x%(WORLD_SIZE as f32);
        let warp_y = y%(WORLD_SIZE as f32);

        return (warp_x, warp_y);
    }
}
