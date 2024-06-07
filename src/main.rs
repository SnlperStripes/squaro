mod comm;
mod enemy;
mod state;
mod text;
mod projectile;

use ggez::{ContextBuilder, GameResult};
use ggez::event;
use state::MainState;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use comm::Comm;
use env_logger;  // Import the logger crate

fn main() -> GameResult {
    // Initialize the logger
    env_logger::init();

    // Create a context and event loop
    let (ctx, event_loop) = ContextBuilder::new("Squaro", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Squaro!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Initialize Tokio runtime and Comm
    let rt = Arc::new(Runtime::new().unwrap());
    let ws_stream = Arc::new(Mutex::new(None));
    let comm = Comm { ws_stream: ws_stream.clone() };
    
    // Start WebSocket listener
    rt.spawn(Comm::start_listener("127.0.0.1:5555", ws_stream));

    // Create the main state
    let state = MainState::new(comm, rt)?;

    // Run the game loop
    event::run(ctx, event_loop, state)
}
