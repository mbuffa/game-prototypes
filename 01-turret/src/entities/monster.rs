use macroquad::prelude::*;

pub struct Monster {
  identifier: String,
  collider: Circle,
  velocity: f32,
  alive: bool,
  health: i16
}

impl Monster {
  pub fn new(identifier: String, x: f32, y: f32, velocity: f32) -> Self {
    Self {
      identifier,
      collider: Circle::new(x, y, 20f32),
      velocity,
      alive: true,
      health: 100
    }
  }

  pub fn draw(&self, texture: &Texture2D) {
    draw_texture(
      *texture,
      self.collider.x - (texture.width() / 2f32),
      self.collider.y - (texture.height() / 2f32),
      Color { r: 1.0, g: 1.0, b: 1.0, a: self.health as f32 / 100f32 }
    );

    // Debug
    // draw_circle(self.collider.x, self.collider.y, self.collider.r, YELLOW);
  }

  pub fn update(&mut self, elapsed: f32) {
    self.collider.y += self.velocity * elapsed;
  }

  pub fn identifier(&self) -> &String { &self.identifier }

  pub fn is_alive(&self) -> bool {
    self.alive
  }

  pub fn destroy(&mut self) {
    self.alive = false;
  }

  pub fn hit(&mut self, amount: u8) {
    if self.health <= 0 {
      self.destroy();
    } else {
      self.health -= amount as i16;
    }
  }

  pub fn get_collider(&self) -> &Circle {
    &self.collider
  }
}