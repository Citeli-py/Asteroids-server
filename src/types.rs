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

#[derive(Clone)]
pub struct Client {
    pub id: ClientId,
    pub writer: ArcWriter,
    pub reader: ArcReader,
}

impl Client {
    pub fn new(writer: SplitSink<WSStream, Message>, reader: SplitStream<WSStream>) -> Self {
        Self {
            id: Uuid::new_v4(),
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
        }
    }
}
