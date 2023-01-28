use macroquad::prelude::*;

pub struct Missile {
  collider: Circle,
  angle: f32,
  velocity: f32,
  alive: bool
}

impl Missile {
  pub fn new(x: f32, y: f32, angle: f32, velocity: f32) -> Self {
    Self {
      collider: Circle::new(x, y, 2f32),
      angle,
      velocity,
      alive: true
    }
  }

  pub fn draw(&self, texture: &Texture2D) {
    draw_texture_ex(
      *texture,
      self.collider.x - (texture.width() / 2f32),
      self.collider.y - (texture.height() / 2f32),
      WHITE,
      DrawTextureParams { rotation: (self.angle - 90f32).to_radians(), dest_size: None, source: None, flip_x: false, flip_y: false, pivot: None }//Some(Vec2::new(self.pivot_x, self.pivot_y)) }
    );

    // // Debug
    // draw_circle(self.collider.x, self.collider.y, self.collider.r, BLUE);
  }

  pub fn update(&mut self, elapsed: f32) {
    self.collider.x += self.velocity * (self.angle).to_radians().cos() * elapsed;
    self.collider.y += self.velocity * (self.angle).to_radians().sin() * elapsed;
  }

  pub fn is_alive(&self) -> bool { self.alive }

  pub fn destroy(&mut self) {
    self.alive = false;
  }

  pub fn get_collider(&self) -> &Circle {
    &self.collider
  }
}
