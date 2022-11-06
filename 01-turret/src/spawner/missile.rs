use macroquad::prelude::*;

pub struct Missile {
  pub rect: Rect,
  angle: f32,
  velocity: f32
}

impl Missile {
  pub fn new(x: f32, y: f32, angle: f32, velocity: f32) -> Self {
    Self {
      rect: Rect::new(x, y, 2f32, 20f32),
      angle,
      velocity
    }
  }

  pub fn draw(&self, texture: Texture2D) {
    // draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    draw_texture_ex(
      texture,
      self.rect.x - (texture.width() / 2f32),
      self.rect.y - (texture.height()),
      WHITE,
      DrawTextureParams { rotation: (self.angle + 90f32).to_radians(), dest_size: None, source: None, flip_x: false, flip_y: false, pivot: None } // Some(Vec2::new(self.rect.x, self.rect.y)) }
    );
  }

  pub fn update(&mut self, elapsed: f32) {
    self.rect.x += self.velocity * self.angle.to_radians().cos() * elapsed;
    self.rect.y += self.velocity * self.angle.to_radians().sin() * elapsed;
  }
}
