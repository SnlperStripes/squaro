use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Rect};
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
}

impl Enemy {
    pub fn new(x: f32, y: f32, shape: Shape) -> Self {
        Enemy { x, y, shape }
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

    pub fn check_collisions(&mut self, main_rect: &Rect, score: &mut i32) {
        self.enemies.retain(|enemy| {
            let enemy_rect = Rect::new(enemy.x, enemy.y, 50.0, 50.0);
            if main_rect.overlaps(&enemy_rect) {
                *score += 1;
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
    pub spawner: Spawner,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let spawner = Spawner::new();
        let s = MainState {
            pos_x: 350.0,
            pos_y: 250.0,
            score: 0,
            spawner,
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
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.spawner.spawn();
        let main_rect = Rect::new(self.pos_x, self.pos_y, 50.0, 50.0);
        self.spawner.check_collisions(&main_rect, &mut self.score);

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

        // Draw the score
        crate::text::draw_text(ctx, &format!("Score: {}", self.score))?;

        // Present the drawing
        graphics::present(ctx)?;
        Ok(())
    }
}
