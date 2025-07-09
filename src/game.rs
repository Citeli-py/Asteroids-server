use crate::server::Server;
use crate::types::{ClientId};
use std::time::Duration;
use tokio::time::sleep;

pub struct GameManager {
    server: Server,
}

impl GameManager {
    pub async fn new(addr: &str) -> Self {
        let server = Server::new(addr).await;
        Self { server }
    }

    pub async fn run(&mut self) {
        // Inicia o servidor WebSocket em background
        self.server.listen().await;

        // Loop principal da lógica do jogo
        loop {
            // Exemplo: obter todos os clientes conectados
            let clients = self.server.get_clients().await;
            println!("Clientes conectados: {}", clients.len());

            // Exemplo: enviar uma mensagem para todos os clientes
            self.server.broadcast("tick".into()).await;

            // Espera 5s entre os ticks
            sleep(Duration::from_secs(5)).await;
        }
    }
}
