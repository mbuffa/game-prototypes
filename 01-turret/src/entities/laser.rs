use macroquad::prelude::*;

pub struct Laser {
  origin: Vec2,
  target: Vec2
}

impl Laser {
  pub fn new(ox: f32, oy: f32, tx: f32, ty: f32) -> Self {
    // println!("New laser for {} {} {} {}", ox, oy, tx, ty);
    Self {
      origin: Vec2::new(ox, oy),
      target: Vec2::new(tx, ty),
    }
  }

  pub fn draw(&self) {
    draw_line(self.origin.x, self.origin.y, self.target.x, self.target.y, 2f32, WHITE);
  }

  pub fn get_target_position(&self) -> &Vec2 { &self.target }
}