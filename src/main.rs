use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Rect};

struct MainState;

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState;
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Set the background color
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // Create a rectangle (square)
        let square = Rect::new(350.0, 250.0, 100.0, 100.0);

        // Set the color and draw the square
        let color = Color::from_rgb(255, 0, 0);
        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), square, color)?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Present the drawing
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // Create a context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Hello, ggez!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create the main state
    let state = MainState::new()?;

    // Run the game loop
    event::run(ctx, event_loop, state)
}
