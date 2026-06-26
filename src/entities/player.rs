use crate::entities::bullet::Bullet;
use crate::entities::traits::warp_object::WarpObject;
use crate::entities::traits::collision_object::CollisionObject;
use crate::entities::hitbox::{HitBox, EntityKind, LAYER_BULLET, LAYER_ASTEROID};
use crate::types::{ClientId, TICK_RATE, WORLD_SIZE};
use rand::Rng;

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
    deceleration: f32,
    input_buffer: Vec<CMD>,
    buffer_size: usize,
    client_id: ClientId,
    is_destroyed: bool,

    score: u32,
    shot_cooldown: u32,
    shot_counter: u32,
}

impl CollisionObject for Player {

    fn hitbox(&self) -> HitBox {
        HitBox::circle(
            self.client_id,
            EntityKind::Player,
            (self.x, self.y),
            10.0,
            LAYER_BULLET | LAYER_ASTEROID,
        )
    }
}

impl WarpObject for Player {
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Player {
    /// Jogo: criação sem se preocupar com RNG (entropia).
    pub fn new(client_id: &ClientId) -> Player {
        Self::with_rng(client_id, &mut rand::rng())
    }

    /// Teste/benchmark: RNG injetado, posição de spawn reproduzível.
    pub fn with_rng(client_id: &ClientId, rng: &mut impl Rng) -> Player {

        let tick = TICK_RATE as f32;
        Player {
            x: rng.random_range(0.0..(WORLD_SIZE as f32)),
            y: rng.random_range(0.0..(WORLD_SIZE as f32)),
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            turn_speed: 2.0 / tick,
            acceleration: 7.0 / tick,
            deceleration: 2.0 / tick,
            input_buffer: vec![],
            buffer_size: 2,
            client_id: client_id.clone(),

            shot_cooldown: (0.4 * tick) as u32,
            shot_counter: (1.0 * tick) as u32,

            score: 0,
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

        self.apply_move_commands();
        self.movement();

        let bullet = self.apply_fire_commands();
        self.shot_counter += 1;
        self.clear_input_buffer();

        return bullet;
    }

    fn apply_move_commands(&mut self,) {
        // Primeiro: processa todos os comandos
        for cmd in self.input_buffer.iter() {
            match cmd {
                CMD::UP => {
                    let max_speed: f32 = 10.0;
                    // Aceleração na direção do ângulo
                    self.vx += self.acceleration * self.angle.cos();
                    self.vy += self.acceleration * self.angle.sin();

                    // Limita a velocidade pela magnitude do vetor (cap em qualquer direção)
                    let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
                    if speed > max_speed {
                        self.vx = self.vx / speed * max_speed;
                        self.vy = self.vy / speed * max_speed;
                    }
                }
                CMD::LEFT => {
                    self.angle -= self.turn_speed;
                }
                CMD::RIGHT => {
                    self.angle += self.turn_speed;
                }
                _ => {}
            }
        }
    }

    fn movement(&mut self,) {
        // Atualiza posição com velocidade
        self.x += self.vx;
        self.y += self.vy;
        
        // Desaceleração: reduz a velocidade de forma linear até parar no zero
        // (sem inverter a direção, diferente da fricção multiplicativa).
        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if speed > 0.0 {
            let new_speed = (speed - self.deceleration).max(0.0);
            let factor = new_speed / speed;
            self.vx *= factor;
            self.vy *= factor;
        }

        // Warp
        (self.x, self.y) = self.warp();
    }

    fn apply_fire_commands(&mut self) -> Option<Bullet> {
        
        if !self.can_shoot() {
            return None;
        }

        if self.input_buffer.contains(&CMD::SHOT){
            return self.fire();
        }

        None
    }

    fn fire(&mut self,) -> Option<Bullet> {

        let v0 = f32::sqrt(self.vx*self.vx + self.vy*self.vy);
        self.shot_counter = 0;

        //Knockback
        // let tick = TICK_RATE as f32;
        // let knockback = 10.0/tick;
        // self.vy -= knockback*self.angle.sin();
        // self.vx -= knockback*self.angle.cos();

        Some(Bullet::new(self.client_id, self.x, self.y, v0,  self.angle))
    }

    pub fn to_json(&self) -> String {

        format!("{{ \"id\":\"{}\", \"x\": {}, \"y\":{}, \"angle\": {}, \"is_destroyed\": {}, \"score\": {} }}",
                self.client_id, self.x, self.y, self.angle, self.is_destroyed, self.score)
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