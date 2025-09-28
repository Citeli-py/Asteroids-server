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
        Player {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            turn_speed: 2.0 / TICK_RATE as f32,
            acceleration: 1.0 / TICK_RATE as f32,
            friction: 0.999,
            input_buffer: vec![],
            buffer_size: 2,
            client_id: client_id.clone(),

            shot_cooldown: 1 * TICK_RATE as u32,
            shot_counter: 1 * TICK_RATE as u32,

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
        let is_fired;
        let mut new_bullet: Option<Bullet> = None;

        (self.x, self.y, self.vx, self.vy, self.angle, is_fired) = self.apply_commands(&self.input_buffer);
        
        if is_fired && self.can_shoot() {
            new_bullet = Some(Bullet::new(self.client_id, self.x, self.y, self.angle));
            self.shot_counter = 0;
        }

        self.clear_input_buffer();
        self.shot_counter += 1;

        new_bullet
    }

    fn apply_commands(&self, commands: &Vec<CMD>) -> (f32, f32, f32, f32, f32, bool) {
        let (mut x, mut y, mut vx, mut vy, mut angle) =
            (self.x, self.y, self.vx, self.vy, self.angle);

        let mut is_fired = false;

        // Primeiro: processa todos os comandos
        for cmd in commands.iter() {
            match cmd {
                CMD::UP => {
                    // Aceleração na direção do ângulo
                    vx += self.acceleration * angle.cos();
                    vy += self.acceleration * angle.sin();
                }
                CMD::LEFT => {
                    angle -= self.turn_speed;
                }
                CMD::RIGHT => {
                    angle += self.turn_speed;
                }
                CMD::SHOT => {
                    is_fired = true;
                }
                CMD::NONE => {}
            }
        }

        // DEPOIS do loop: aplica física uma vez
        // Atualiza posição com velocidade
        x += vx;
        y += vy;
        
        // Aplica fricção (reduz velocidade gradualmente)
        vx *= self.friction;
        vy *= self.friction;

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