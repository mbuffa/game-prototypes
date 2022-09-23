use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Obstacle {
  pub rect: Rect
}

impl Obstacle {
  pub fn new() -> Self {
    Self {
      rect: Rect::new(screen_width() + 100f32, (screen_height() + 40f32) * 0.5f32, 40f32, 40f32)
    }
  }
  
  pub fn update(&mut self, elapsed: f32) {
    self.rect.x -= 1000f32 * elapsed;
  }
  
  pub fn draw(& self) {
    draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED)
  }
}

impl std::fmt::Display for Obstacle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.rect.x, self.rect.y)
  }
}
