use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::timer;
use std::time::Duration;
use rand::Rng;

#[derive(Debug)]
pub enum Shape {
    Square,
    Circle,
}

#[derive(Debug)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub shape: Shape,
    pub dx: f32,
    pub dy: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, shape: Shape) -> Self {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-2.0..2.0);
        let dy = rng.gen_range(-2.0..2.0);
        Enemy { x, y, shape, dx, dy }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        // Bounce off walls
        if self.x <= 0.0 || self.x >= 750.0 {
            self.dx = -self.dx;
        }
        if self.y <= 0.0 || self.y >= 550.0 {
            self.dy = -self.dy;
        }
    }
}

pub struct Spawner {
    enemies: Vec<Enemy>,
}

impl Spawner {
    pub fn new() -> Self {
        Spawner {
            enemies: Vec::new(),
        }
    }

    pub fn spawn(&mut self) {
        let mut rng = rand::thread_rng();
        while self.enemies.len() < 5 {
            let x = rng.gen_range(0.0..750.0);
            let y = rng.gen_range(0.0..550.0);
            let shape = if rng.gen_bool(0.5) {
                Shape::Square
            } else {
                Shape::Circle
            };
            self.enemies.push(Enemy::new(x, y, shape));
        }
    }

    pub fn update_enemies(&mut self) {
        for enemy in &mut self.enemies {
            enemy.update();
        }
    }

    pub fn check_collisions(&mut self, main_rect: &Rect, score: &mut i32, health: &mut i32) {
        self.enemies.retain(|enemy| {
            let enemy_rect = Rect::new(enemy.x, enemy.y, 50.0, 50.0);
            if main_rect.overlaps(&enemy_rect) {
                *score += 1;
                *health -= 1;
                false // Remove the enemy
            } else {
                true // Keep the enemy
            }
        });
    }

    pub fn get_closest_enemy(&self, x: f32, y: f32) -> Option<&Enemy> {
        self.enemies
            .iter()
            .min_by(|a, b| {
                let dist_a = (a.x - x).powi(2) + (a.y - y).powi(2);
                let dist_b = (b.x - x).powi(2) + (b.y - y).powi(2);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }
}

pub struct MainState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub score: i32,
    pub health: i32,
    pub spawner: Spawner,
    pub freeze_timer: Option<Duration>,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let spawner = Spawner::new();
        let s = MainState {
            pos_x: 350.0,
            pos_y: 250.0,
            score: 0,
            health: 15,
            spawner,
            freeze_timer: None,
        };
        Ok(s)
    }

    fn move_towards(&mut self, target_x: f32, target_y: f32) {
        let dx = target_x - self.pos_x;
        let dy = target_y - self.pos_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 1.0 {
            self.pos_x += dx / distance;
            self.pos_y += dy / distance;
        }
    }

    fn handle_freeze(&mut self, ctx: &mut Context) {
        if let Some(start_time) = self.freeze_timer {
            if timer::time_since_start(ctx) - start_time < Duration::new(3, 0) {
                // If the freeze time hasn't elapsed, continue freezing
                return;
            } else {
                // Freeze time elapsed, reset health and stop freezing
                self.health = 15;
                self.freeze_timer = None;
            }
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.health <= 0 {
            if self.freeze_timer.is_none() {
                self.freeze_timer = Some(timer::time_since_start(ctx));
            }
            self.handle_freeze(ctx);
            return Ok(());
        }

        self.spawner.spawn();
        self.spawner.update_enemies();
        let main_rect = Rect::new(self.pos_x, self.pos_y, 50.0, 50.0);
        self.spawner.check_collisions(&main_rect, &mut self.score, &mut self.health);

        if let Some(enemy) = self.spawner.get_closest_enemy(self.pos_x, self.pos_y) {
            self.move_towards(enemy.x, enemy.y);
        }

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
            };
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        // Draw the score at the top right
        crate::text::draw_text(ctx, &format!("Score: {}", self.score), [700.0, 10.0])?;

        // Draw the health at the top left
        crate::text::draw_text(ctx, &format!("Health: {}", self.health), [10.0, 10.0])?;

        // Present the drawing
        graphics::present(ctx)?;
        Ok(())
    }
}
