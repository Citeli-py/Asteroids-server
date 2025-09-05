use crate::{bullet::Bullet, collision_object::CollisionObject, types::{ClientId, TICK_RATE}};

#[derive(PartialEq, Clone, Debug)]
pub enum CMD {
    UP,
    LEFT,
    RIGHT,
    SHOT,
    NONE
}

#[derive(Clone)]
pub struct Player {
    x: f32,
    y: f32,
    angle: f32,
    vx: f32,
    vy: f32,
    turn_speed: f32,
    acceleration: f32,
    friction: f32,
    input_buffer: Vec<CMD>,
    buffer_size: usize,
    client_id: ClientId,
    is_destroyed: bool
}

impl CollisionObject for Player {
    
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn radius(&self) -> f32 {
        10.0
    }
}

impl Player {
    pub fn new(client_id: &ClientId) -> Player {
        Player {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            turn_speed: 0.2,
            acceleration: 0.2,
            friction: 0.9999,
            input_buffer: vec![],
            buffer_size: 2,
            client_id: client_id.clone(),

            is_destroyed: false,
        }
    }

    pub fn push_command(&mut self, cmd: CMD) {
        if self.input_buffer.len() >= self.buffer_size {
            self.input_buffer.remove(0); // Remove o comando mais antigo
        }
        self.input_buffer.push(cmd);
    }

    pub fn clear_input_buffer(&mut self, ) {
        self.input_buffer = vec![CMD::NONE];
    }

    pub fn update(&mut self) -> Option<Bullet>{
        let dt = 1.0 / TICK_RATE as f32;
        let is_fired;
        let mut new_bullet: Option<Bullet> = None;

        (self.x, self.y, self.vx, self.vy, self.angle, is_fired) = self.apply_commands(&self.input_buffer, dt);
        
        if is_fired {
            new_bullet = Some(Bullet::new(self.client_id, self.x, self.y, self.angle));
        }

        self.clear_input_buffer();

        new_bullet
    }

    fn apply_commands(&self, commands: &Vec<CMD>, dt: f32) -> (f32, f32, f32, f32, f32, bool) {
        let (mut x, mut y, mut vx, mut vy, mut angle) =
            (self.x, self.y, self.vx, self.vy, self.angle);

        let mut is_fired = false;
        // Processa cada comando
        for cmd in commands.into_iter() {
            match cmd {
                CMD::UP => {
                    vx += self.acceleration * f32::cos(angle) * dt;
                    vy += self.acceleration * f32::sin(angle) * dt;
                }
                CMD::LEFT => {
                    angle -= self.turn_speed * dt;
                }
                CMD::RIGHT => {
                    angle += self.turn_speed * dt;
                }
                CMD::SHOT => {
                    is_fired = true;
                }
                CMD::NONE => {
                    // Nada - apenas aplica a física básica
                }
            }

            // Aplica física mesmo sem comandos
            x += vx * dt;
            y += vy * dt;
            
            // Aplica fricção
            vx *= self.friction;
            vy *= self.friction;
        }

        (x, y, vx, vy, angle, is_fired)
    }

    pub fn to_json(&self, ) -> String {
        
        format!("{{ \"id\":\"{}\", \"x\": {}, \"y\":{}, \"angle\": {}, \"is_destroyed\": {} }}", 
                self.client_id, self.x, self.y, self.angle, self.is_destroyed)
    }

    pub fn destroy(&mut self, ){
        self.is_destroyed = true;
    }

    pub fn get_position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.angle)
    }

    pub fn get_id(&self) -> ClientId {
        self.client_id.clone()
    }

}