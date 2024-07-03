mod enemy;
mod state;
mod text;
mod projectile;
mod rl_interface;

use crate::rl_interface::RLInterface;
use cpython::Python;
use ggez::{ContextBuilder, GameResult};
use ggez::event;
use state::MainState;

fn main() -> GameResult {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let rl_interface = RLInterface::new(py).expect("Failed to create RL interface");

    // Create a context and event loop
    let (ctx, event_loop) = ContextBuilder::new("Squaro", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Squaro!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create the main state
    let state = MainState::new(rl_interface)?;

    // Run the game loop
    event::run(ctx, event_loop, state)
}
