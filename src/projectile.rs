#[derive(Debug)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Projectile {
    pub fn new(x: f32, y: f32, target_x: f32, target_y: f32) -> Self {
        let speed = 5.0;
        let dx = target_x - x;
        let dy = target_y - y;
        let distance = (dx * dx + dy * dy).sqrt();
        let dx = dx / distance * speed;
        let dy = dy / distance * speed;

        Projectile { x, y, dx, dy }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}
