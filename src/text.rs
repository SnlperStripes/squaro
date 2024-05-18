use ggez::{Context, GameResult};
use ggez::graphics::{self, Text, DrawParam, Color};

pub fn draw_text(ctx: &mut Context, text: &str) -> GameResult {
    let display_text = Text::new(text);
    let draw_params = DrawParam::default()
        .dest([700.0, 10.0]) // Position the text at the top right
        .color(Color::WHITE); // Set text color to white

    graphics::draw(ctx, &display_text, draw_params)?;
    Ok(())
}
