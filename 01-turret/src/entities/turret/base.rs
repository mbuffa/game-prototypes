use macroquad::prelude::*;

pub struct Base {
  x: f32,
  y: f32
}

impl Base {
  pub fn new(x: f32, y: f32) -> Self {
    Self {x, y}
  }

  pub fn draw(&self, texture: &Texture2D) {
    draw_texture(*texture, self.x - ((*texture).width() / 2f32), self.y - ((*texture).height() / 2f32), WHITE);
  }

  pub fn y(&self) -> &f32 {
    &self.y
  }
}