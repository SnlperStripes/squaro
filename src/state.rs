use crate::enemy::{Spawner, Shape};
use crate::projectile::Projectile;
use crate::rl_interface::RLInterface;
use cpython::Python;
use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::mouse::MouseButton;

pub struct MainState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub score: i32,
    pub spawner: Spawner,
    pub paused: bool,
    pub projectiles: Vec<Projectile>,
    pub rl_interface: RLInterface,
}

impl MainState {
    pub fn new(rl_interface: RLInterface) -> GameResult<MainState> {
        let spawner = Spawner::new();
        let s = MainState {
            pos_x: 350.0,
            pos_y: 250.0,
            score: 0,
            spawner,
            paused: false,
            projectiles: Vec::new(),
            rl_interface,
        };
        Ok(s)
    }

    fn update_projectiles(&mut self) {
        for projectile in &mut self.projectiles {
            projectile.update();
        }

        self.projectiles.retain(|projectile| {
            projectile.x >= 0.0 && projectile.x <= 800.0 && projectile.y >= 0.0 && projectile.y <= 600.0
        });
    }

    fn check_projectile_collisions(&mut self) {
        for projectile in &self.projectiles {
            self.spawner.enemies.retain(|enemy| {
                let enemy_rect = Rect::new(enemy.x, enemy.y, 50.0, 50.0);
                let projectile_rect = Rect::new(projectile.x, projectile.y, 5.0, 5.0);
                if projectile_rect.overlaps(&enemy_rect) {
                    self.score += 1;
                    false // Remove the enemy
                } else {
                    true // Keep the enemy
                }
            });
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.paused {
            return Ok(());
        }

        self.spawner.spawn();
        self.spawner.update_enemies(self.pos_x, self.pos_y);  // Pass player position
        self.update_projectiles();
        let main_rect = Rect::new(self.pos_x, self.pos_y, 50.0, 50.0);
        self.spawner.check_collisions(&main_rect, &mut self.score); // Check collisions with enemies
        self.check_projectile_collisions(); // Check collisions with projectiles

        let gil = Python::acquire_gil();
        let py = gil.python();
        let state = format!("{{\"player_x\": {}, \"player_y\": {}, \"score\": {}}}", self.pos_x, self.pos_y, self.score);
        match self.rl_interface.compute_action(py, &state) {
            Ok(action) => match action.as_str() {
                "up" => self.pos_y = (self.pos_y - 5.0).max(0.0),
                "down" => self.pos_y = (self.pos_y + 5.0).min(550.0),
                "left" => self.pos_x = (self.pos_x - 5.0).max(0.0),
                "right" => self.pos_x = (self.pos_x + 5.0).min(750.0),
                _ => (),
            },
            Err(e) => println!("Python error: {:?}", e),
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::W => self.pos_y = (self.pos_y - 5.0).max(0.0),
            KeyCode::A => self.pos_x = (self.pos_x - 5.0).max(0.0),
            KeyCode::S => self.pos_y = (self.pos_y + 5.0).min(550.0),
            KeyCode::D => self.pos_x = (self.pos_x + 5.0).min(750.0),
            KeyCode::P => self.paused = !self.paused, // Toggle the paused state
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let projectile = Projectile::new(self.pos_x + 25.0, self.pos_y + 25.0, x, y);
            self.projectiles.push(projectile);
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

        // Create and draw the enemies
        for enemy in &self.spawner.enemies {
            let mesh = match enemy.shape {
                Shape::Square => {
                    let square = Rect::new(enemy.x, enemy.y, 50.0, 50.0);
                    graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), square, Color::from_rgb(0, 0, 255))?
                }
                Shape::Circle => {
                    graphics::Mesh::new_circle(ctx, DrawMode::fill(), [enemy.x + 25.0, enemy.y + 25.0], 25.0, 0.1, Color::from_rgb(0, 255, 0))?
                }
                Shape::Triangle => {
                    // Draw a triangle
                    let points = [
                        [enemy.x + 25.0, enemy.y], // Top point
                        [enemy.x, enemy.y + 50.0], // Bottom left point
                        [enemy.x + 50.0, enemy.y + 50.0], // Bottom right point
                    ];
                    graphics::Mesh::new_polygon(ctx, DrawMode::fill(), &points, Color::from_rgb(255, 255, 0))?
                }
            };
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        // Create and draw the projectiles
        for projectile in &self.projectiles {
            let rect = Rect::new(projectile.x, projectile.y, 5.0, 5.0);
            let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::from_rgb(255, 255, 255))?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        // Draw the score at the top right
        crate::text::draw_text(ctx, &format!("Score: {}", self.score), [700.0, 10.0])?;

        // If the game is paused, draw the "Paused" text
        if self.paused {
            crate::text::draw_text(ctx, "Paused", [350.0, 300.0])?;
        }

        // Present the drawing
        graphics::present(ctx)?;
        Ok(())
    }
}
