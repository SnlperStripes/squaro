use crate::enemy::{Spawner, Shape};
use crate::projectile::Projectile;
use crate::rl_interface::RLInterface;
use crate::text;
use cpython::Python;
use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::mouse::MouseButton;
use std::time::{Duration, Instant};


pub struct MainState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub score: i32,
    pub previous_score: i32,
    pub spawner: Spawner,
    pub paused: bool,
    pub projectiles: Vec<Projectile>,
    pub rl_interface: RLInterface,
    pub running: bool,
    pub last_score_print: Instant
}

impl MainState {
    pub fn new(rl_interface: RLInterface) -> GameResult<MainState> {
        let spawner = Spawner::new();
        let s = MainState {
            pos_x: 350.0,
            pos_y: 250.0,
            score: 0,
            previous_score: 0,
            spawner,
            paused: false,
            projectiles: Vec::new(),
            rl_interface,
            running: true,
            last_score_print: Instant::now(), // Initialize the timer
        };
        Ok(s)
    }

    pub fn update_projectiles(&mut self) {
        for projectile in &mut self.projectiles {
            projectile.update();
        }

        self.projectiles.retain(|projectile| {
            projectile.x >= 0.0 && projectile.x <= 800.0 && projectile.y >= 0.0 && projectile.y <= 600.0
        });
    }

    pub fn check_projectile_collisions(&mut self) {
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

    pub fn get_state(&self) -> String {
        let enemy_positions: Vec<String> = self.spawner.enemies.iter().map(|e| {
            let enemy_type = match e.shape {
                Shape::Square => "square",
                Shape::Circle => "circle",
                Shape::Triangle => "triangle",
            };
            let relative_x = e.x - self.pos_x;
            let relative_y = e.y - self.pos_y;
            let distance = ((relative_x * relative_x) + (relative_y * relative_y)).sqrt();
            format!("{{\"x\": {}, \"y\": {}, \"type\": \"{}\", \"distance\": {}}}", relative_x, relative_y, enemy_type, distance)
        }).collect();
        let enemy_positions_str = enemy_positions.join(", ");
        format!("{{\"player_x\": {}, \"player_y\": {}, \"score\": {}, \"enemies\": [{}]}}", self.pos_x, self.pos_y, self.score, enemy_positions_str)
    }
    

    pub fn perform_action(&mut self, action: &str) {
        match action {
            "up" => self.pos_y = (self.pos_y - 5.0).max(0.0),
            "down" => self.pos_y = (self.pos_y + 5.0).min(550.0),
            "left" => self.pos_x = (self.pos_x - 5.0).max(0.0),
            "right" => self.pos_x = (self.pos_x + 5.0).min(750.0),
            _ => (),
        }
    }
    
    pub fn get_reward(&mut self) -> f32 {
        let reward = (self.score - self.previous_score) as f32;
        self.previous_score = self.score;
        reward
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
        let state = self.get_state();
        let action = match self.rl_interface.compute_action(py, &state) {
            Ok(action) => action,
            Err(e) => {
                println!("Python error: {:?}", e);
                return Ok(());
            },
        };
        self.perform_action(&action);

        // Call the learn method
        let next_state = self.get_state();
        let reward = self.get_reward();
        self.rl_interface.learn(py, &state, &action, reward, &next_state).expect("Failed to learn");

        // Decay epsilon
        if let Err(e) = self.rl_interface.decay_epsilon(py) {
            println!("Failed to decay epsilon: {:?}", e);
        }

        // Log the score periodically
        if self.last_score_print.elapsed() >= Duration::from_secs(5) {
            println!("Current score: {}", self.score);
            self.last_score_print = Instant::now();
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
        text::draw_text(ctx, &format!("Score: {}", self.score), [700.0, 10.0])?;

        // If the game is paused, draw the "Paused" text
        if self.paused {
            text::draw_text(ctx, "Paused", [350.0, 300.0])?;
        }

        // Present the drawing
        graphics::present(ctx)?;
        Ok(())
    }
}
