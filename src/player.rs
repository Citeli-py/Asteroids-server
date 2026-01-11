use crate::bullet::Bullet;
use crate::collision_object::CollisionObject;
use crate::types::{ClientId, TICK_RATE, WORLD_SIZE};

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
    is_destroyed: bool,

    shot_cooldown: u32,
    shot_counter: u32,
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

        let tick = TICK_RATE as f32;
        Player {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            turn_speed: 2.0 / tick,
            acceleration: 1.0 / tick,
            friction: 0.999,
            input_buffer: vec![],
            buffer_size: 2,
            client_id: client_id.clone(),

            shot_cooldown: (0.8 * tick) as u32,
            shot_counter: (1.0 * tick) as u32,

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

    pub fn can_shoot(&self,) -> bool {
        self.shot_cooldown <= self.shot_counter
    }

    pub fn update(&mut self) -> Option<Bullet>{
        let mut new_bullet: Option<Bullet> = None;

        let is_fired = self.apply_commands();
        
        if is_fired && self.can_shoot() {
            let v0 = f32::sqrt(self.vx*self.vx + self.vy*self.vy);
            new_bullet = Some(Bullet::new(self.client_id, self.x, self.y, v0,  self.angle));
            self.shot_counter = 0;

            //Knockback
            let tick = TICK_RATE as f32;
            let knockback = 10.0/tick;
            self.vy -= knockback*self.angle.sin();
            self.vx -= knockback*self.angle.cos();
        }

        self.clear_input_buffer();
        self.shot_counter += 1;

        new_bullet
    }

    fn apply_commands(&mut self,) -> bool {

        let mut is_fired = false;

        // Primeiro: processa todos os comandos
        for cmd in self.input_buffer.iter() {
            match cmd {
                CMD::UP => {
                    // Aceleração na direção do ângulo
                    self.vx += self.acceleration * self.angle.cos();
                    self.vy += self.acceleration * self.angle.sin();
                }
                CMD::LEFT => {
                    self.angle -= self.turn_speed;
                }
                CMD::RIGHT => {
                    self.angle += self.turn_speed;
                }
                CMD::SHOT => {
                    is_fired = true;
                }
                CMD::NONE => {}
            }
        }

        // DEPOIS do loop: aplica física uma vez
        // Atualiza posição com velocidade
        self.x += self.vx;
        self.y += self.vy;
        
        // Aplica fricção (reduz velocidade gradualmente)
        self.vx *= self.friction;
        self.vy *= self.friction;

        // Warp
        self.x = self.x%(WORLD_SIZE as f32);
        self.y = self.y%(WORLD_SIZE as f32);

        is_fired
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