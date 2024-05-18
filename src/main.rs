mod state;

use ggez::{ContextBuilder, GameResult};
use ggez::event;
use state::MainState;

fn main() -> GameResult {
    // Create a context and event loop
    let (ctx, event_loop) = ContextBuilder::new("Squaro", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Squaro!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create the main state
    let state = MainState::new()?;

    // Run the game loop
    event::run(ctx, event_loop, state)
}
