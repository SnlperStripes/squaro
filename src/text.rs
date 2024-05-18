use ggez::{Context, GameResult};
use ggez::graphics::{self, Text, DrawParam, Color};

pub fn draw_text(ctx: &mut Context, text: &str, position: [f32; 2]) -> GameResult {
    let display_text = Text::new(text);
    let draw_params = DrawParam::default()
        .dest(position) // Position the text
        .color(Color::WHITE); // Set text color to white

    graphics::draw(ctx, &display_text, draw_params)?;
    Ok(())
}
