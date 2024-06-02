use crate::enemy::{Spawner, Shape};
use crate::projectile::Projectile;
use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::mouse::MouseButton;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub player_x: i32,
    pub player_y: i32,
    pub enemies: Vec<(i32, i32)>,
    pub projectiles: Vec<(i32, i32)>,
}

pub struct QLearningAgent {
    q_table: HashMap<(State, Action), f32>,
    learning_rate: f32,
    discount_factor: f32,
    exploration_rate: f32,
}

impl QLearningAgent {
    pub fn new(learning_rate: f32, discount_factor: f32, exploration_rate: f32) -> Self {
        QLearningAgent {
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            exploration_rate,
        }
    }

    pub fn choose_action(&self, state: &State) -> Action {
        if rand::random::<f32>() < self.exploration_rate {
            // Explore: choose a random action
            let actions = [Action::Up, Action::Down, Action::Left, Action::Right];
            actions[rand::thread_rng().gen_range(0..actions.len())].clone()
        } else {
            // Exploit: choose the best action based on the Q-table
            let mut best_action = Action::Up;
            let mut best_value = f32::MIN;
            for action in [Action::Up, Action::Down, Action::Left, Action::Right].iter() {
                let value = *self.q_table.get(&(state.clone(), action.clone())).unwrap_or(&0.0);
                if value > best_value {
                    best_value = value;
                    best_action = action.clone();
                }
            }
            best_action
        }
    }

    pub fn update_q_table(&mut self, state: State, action: Action, reward: f32, next_state: State) {
        let next_best_action = self.choose_action(&next_state);
        let current_q = self.q_table.get(&(state.clone(), action.clone())).unwrap_or(&0.0);
        let next_q = self.q_table.get(&(next_state, next_best_action)).unwrap_or(&0.0);
        let new_q = current_q + self.learning_rate * (reward + self.discount_factor * next_q - current_q);
        self.q_table.insert((state, action), new_q);
    }
}

pub struct MainState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub score: i32,
    pub spawner: Spawner,
    pub paused: bool,
    pub projectiles: Vec<Projectile>,
    pub q_agent: QLearningAgent,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let spawner = Spawner::new();
        let q_agent = QLearningAgent::new(0.1, 0.9, 0.1);
        let s = MainState {
            pos_x: 350.0,
            pos_y: 250.0,
            score: 0,
            spawner,
            paused: false,
            projectiles: Vec::new(),
            q_agent,
        };
        Ok(s)
    }

    fn get_state(&self) -> State {
        State {
            player_x: self.pos_x as i32,
            player_y: self.pos_y as i32,
            enemies: self.spawner.enemies.iter().map(|e| (e.x as i32, e.y as i32)).collect(),
            projectiles: self.projectiles.iter().map(|p| (p.x as i32, p.y as i32)).collect(),
        }
    }

    fn execute_action(&mut self, action: Action) {
        match action {
            Action::Up => self.pos_y = (self.pos_y - 5.0).max(0.0),
            Action::Down => self.pos_y = (self.pos_y + 5.0).min(550.0),
            Action::Left => self.pos_x = (self.pos_x - 5.0).max(0.0),
            Action::Right => self.pos_x = (self.pos_x + 5.0).min(750.0),
        }
    }

    fn update_q_learning(&mut self) {
        let state = self.get_state();
        let action = self.q_agent.choose_action(&state);
        self.execute_action(action.clone());
        let reward = self.score as f32;
        let next_state = self.get_state();
        self.q_agent.update_q_table(state, action, reward, next_state);
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

        self.update_q_learning();

        self.spawner.spawn();
        self.spawner.update_enemies(self.pos_x, self.pos_y);  // Pass player position
        self.update_projectiles();
        let main_rect = Rect::new(self.pos_x, self.pos_y, 50.0, 50.0);
        self.spawner.check_collisions(&main_rect, &mut self.score); // Check collisions with enemies
        self.check_projectile_collisions(); // Check collisions with projectiles

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
