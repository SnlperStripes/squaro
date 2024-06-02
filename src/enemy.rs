use ggez::graphics::Rect;
use rand::Rng;

#[derive(Debug)]
pub enum Shape {
    Square,
    Circle,
    Triangle,
}

#[derive(Debug)]
pub enum EnemyType {
    Chaser,
    Random,
    Avoider,
}

#[derive(Debug)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub shape: Shape,
    pub enemy_type: EnemyType,
    pub dx: f32,
    pub dy: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, shape: Shape) -> Self {
        let enemy_type = match shape {
            Shape::Square => EnemyType::Chaser,
            Shape::Circle => EnemyType::Random,
            Shape::Triangle => EnemyType::Avoider,
        };
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-2.0..2.0);
        let dy = rng.gen_range(-2.0..2.0);
        Enemy { x, y, shape, enemy_type, dx, dy }
    }

    pub fn update(&mut self, player_x: f32, player_y: f32) {
        match self.enemy_type {
            EnemyType::Chaser => {
                let dx = player_x - self.x;
                let dy = player_y - self.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance > 0.0 {
                    self.x += dx / distance;
                    self.y += dy / distance;
                }
            }
            EnemyType::Random => {
                self.x += self.dx;
                self.y += self.dy;
                if self.x <= 0.0 || self.x >= 750.0 {
                    self.dx = -self.dx;
                }
                if self.y <= 0.0 || self.y >= 550.0 {
                    self.dy = -self.dy;
                }
            }
            EnemyType::Avoider => {
                let dx = self.x - player_x;
                let dy = self.y - player_y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 100.0 {
                    self.x += dx / distance;
                    self.y += dy / distance;
                }
            }
        }

        // Ensure the enemy stays within the window boundaries
        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x > 750.0 {
            self.x = 750.0;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y > 550.0 {
            self.y = 550.0;
        }
    }
}

pub struct Spawner {
    pub enemies: Vec<Enemy>,
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
            let shape = match rng.gen_range(0..3) {
                0 => Shape::Square,
                1 => Shape::Circle,
                _ => Shape::Triangle,
            };
            self.enemies.push(Enemy::new(x, y, shape));
        }
    }

    pub fn update_enemies(&mut self, player_x: f32, player_y: f32) {
        for enemy in &mut self.enemies {
            enemy.update(player_x, player_y);
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
}
