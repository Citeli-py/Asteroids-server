mod game;
mod websocket_handler;
mod types;
mod player;
mod bullet;
mod collision_object;
mod warp_object;
mod bullet_collection;
mod player_collection;
mod asteroid_collection;
mod asteroid;
mod client;

use std::time::Duration;
use std::sync::Arc;
use std::net::SocketAddr;
use websocket_handler::WebSocketHandler;

use sysinfo::{
    System
};

use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade},
    http::StatusCode
};

use tower_http::cors::{CorsLayer, Any};

async fn health_check() -> (StatusCode, &'static str) {
    println!("Health check!");
    (StatusCode::OK, "OK")
}

async fn process_info() {
    let mut sys = System::new_all();
    let pid = sysinfo::get_current_pid().unwrap();

    // Primeira leitura (CPU precisa de baseline)
    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), false);

    loop {
        // Atualiza apenas o processo atual
        sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), false);

        if let Some(process) = sys.process(pid) {
            let ram_mb = process.memory() as f64 / 1024.0;
            let cpu_usage = process.cpu_usage();

            println!(
                "Process info: \n\tRAM Used: {:.2} MB \n\tCPU Usage: {:.2}%",
                ram_mb,
                cpu_usage
            );
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}


async fn machine_info() {
    let mut sys = System::new_all();
    
    loop {
        sys.refresh_memory();
        sys.refresh_cpu_all();
        
        // Aguarda um momento para coletar dados de CPU (necessário para precisão)
        std::thread::sleep(std::time::Duration::from_millis(500));
        sys.refresh_cpu_all();
        
        // Informações de memória
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let available_memory = sys.available_memory();
        let memory_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
        
        // Informações de CPU
        let mut cpu_usage = 0.0;
        for cpu in sys.cpus() {
            cpu_usage += cpu.cpu_usage();
        }
        let avg_cpu_usage = cpu_usage / sys.cpus().len() as f32;
        
        // Limitar CPU ao máximo de 0.15 (15%)
        let cpu_display = avg_cpu_usage.min(0.15 * 100.0);
        
        println!(
            "Memory info:
            \tUsed Percentage: {:.2}%
            \tUsed: {} MB
            \tAvailable: {} MB
            \tTotal: {} MB \nCPU info:
            \tUsage: {:.2}%
            \tMax allowed: 15.00%",
            memory_percentage,
            used_memory / 1024 / 1024, // Convertendo para MB
            available_memory / 1024 / 1024, // Convertendo para MB
            total_memory / 1024 / 1024, // Convertendo para MB
            cpu_display
        );
        
        // Aguarda antes da próxima atualização
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() {


    tokio::spawn(machine_info());
    //tokio::spawn(process_info());

    let server = Arc::new(WebSocketHandler::new());

    {
        let broadcast_server = Arc::clone(&server);
        tokio::spawn(async move {
            broadcast_server.start().await
        });
    }

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ws", get(move |ws: WebSocketUpgrade| {
            let server = server.clone();
            async move {
                ws.on_upgrade(move |socket| async move {
                    server.handle_socket(socket).await;
                })
            }
        }))
        .layer(cors);

    let port: u16 = std::env::var("PORT").unwrap_or("8080".into()).parse().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

