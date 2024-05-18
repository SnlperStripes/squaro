use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Rect};
use rand::Rng;

struct MainState {
    pos_x: f32,
    pos_y: f32,
    other_squares: Vec<(f32, f32)>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let other_squares = (0..3)
            .map(|_| {
                (
                    rng.gen_range(0.0..750.0), // Random x position
                    rng.gen_range(0.0..550.0), // Random y position
                )
            })
            .collect();

        let s = MainState {
            pos_x: 350.0,
            pos_y: 250.0,
            other_squares,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::W => self.pos_y -= 5.0,
            KeyCode::A => self.pos_x -= 5.0,
            KeyCode::S => self.pos_y += 5.0,
            KeyCode::D => self.pos_x += 5.0,
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Set the background color
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // Create and draw the main square
        let square = Rect::new(self.pos_x, self.pos_y, 50.0, 50.0);
        let color = Color::from_rgb(255, 0, 0);
        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), square, color)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Create and draw the other squares
        for &(x, y) in &self.other_squares {
            let square = Rect::new(x, y, 50.0, 50.0);
            let color = Color::from_rgb(0, 0, 255); // Different color for other squares
            let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), square, color)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        // Present the drawing
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // Create a context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("Squaro", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Squaro!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create the main state
    let state = MainState::new()?;

    // Run the game loop
    event::run(ctx, event_loop, state)
}
