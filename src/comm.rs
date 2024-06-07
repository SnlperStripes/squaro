use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use log::{info, error};  // Import logging macros

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub enemies: Vec<EnemyState>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnemyState {
    pub x: f32,
    pub y: f32,
    pub shape: String,
    pub enemy_type: String,
}

#[derive(Clone)]
pub struct Comm {
    pub ws_stream: Arc<Mutex<Option<WebSocketStream<TcpStream>>>>,
}

impl Comm {
    pub async fn start_listener(addr: &str, ws_stream: Arc<Mutex<Option<WebSocketStream<TcpStream>>>>) {
        let listener = TcpListener::bind(addr).await.unwrap();
        info!("Listening on: {}", addr);
        while let Ok((stream, _)) = listener.accept().await {
            match accept_async(stream).await {
                Ok(ws) => {
                    info!("Accepted new connection");
                    let mut ws_stream_guard = ws_stream.lock().unwrap();
                    *ws_stream_guard = Some(ws);
                }
                Err(e) => {
                    error!("Error during WebSocket handshake: {:?}", e);
                }
            }
        }
    }

    pub async fn send_state(&self, state: &GameState) {
        let mut ws_stream_guard = self.ws_stream.lock().unwrap();
        if let Some(ref mut ws_stream) = *ws_stream_guard {
            let msg = serde_json::to_string(state).unwrap();
            info!("Sending state: {}", msg);
            if let Err(e) = ws_stream.send(Message::Text(msg)).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    pub async fn receive_action(&self) -> Option<String> {
        let mut ws_stream_guard = self.ws_stream.lock().unwrap();
        if let Some(ref mut ws_stream) = *ws_stream_guard {
            if let Some(Ok(Message::Text(action))) = ws_stream.next().await {
                info!("Received action: {}", action);
                return Some(action);
            }
        }
        None
    }
}
