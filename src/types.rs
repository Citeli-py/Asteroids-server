use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use tokio::net::TcpStream;
use futures_util::stream::{SplitSink, SplitStream};

pub type ClientId = Uuid;
pub type WSStream = WebSocketStream<TcpStream>;
pub type ArcWriter = Arc<Mutex<SplitSink<WSStream, Message>>>;
pub type ArcReader = Arc<Mutex<SplitStream<WSStream>>>;

pub const TICK_RATE: u8 = 32;
pub const WORLD_SIZE:i32 = 2000;