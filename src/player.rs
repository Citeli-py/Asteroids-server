use crate::bullet::{Bullet};
use crate::bullet_collection::BulletCollection;
use crate::warp_object::WarpObject;
use crate::collision_object::CollisionObject;
use crate::types::{ClientId, TICK_RATE};

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

    pub bullets: BulletCollection,
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

impl WarpObject for Player {
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
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

            bullets: BulletCollection::new(),
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

    pub fn update(&mut self){

        self.apply_commands();
        self.movement();
        self.clear_input_buffer();
        self.shot_counter += 1;
    }

    fn apply_commands(&mut self,) {

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

        if is_fired {
            self.fire();
        }
    }

    fn movement(&mut self,) {
        // Atualiza posição com velocidade
        self.x += self.vx;
        self.y += self.vy;
        
        // Aplica fricção (reduz velocidade gradualmente)
        self.vx *= self.friction;
        self.vy *= self.friction;

        // Warp
        (self.x, self.y) = self.warp();
    }

    fn fire(&mut self,) {

        if !self.can_shoot() {
            return;
        }

        let v0 = f32::sqrt(self.vx*self.vx + self.vy*self.vy);
        self.shot_counter = 0;

        //Knockback
        let tick = TICK_RATE as f32;
        let knockback = 10.0/tick;
        self.vy -= knockback*self.angle.sin();
        self.vx -= knockback*self.angle.cos();

        self.bullets.add_bullet(Bullet::new(self.client_id, self.x, self.y, v0,  self.angle));
    }

    pub fn to_json(&self, ) -> String {
        
        format!("{{ \"id\":\"{}\", \"x\": {}, \"y\":{}, \"angle\": {}, \"is_destroyed\": {} }}", 
                self.client_id, self.x, self.y, self.angle, self.is_destroyed)
    }

    pub fn destroy(&mut self, ){
        self.is_destroyed = true;
    }

    pub fn is_destroyed(&self, ) -> bool{
        self.is_destroyed
    }

    pub fn get_id(&self) -> ClientId {
        self.client_id.clone()
    }

}