use macroquad::{prelude::*};

pub struct Base {
  x: f32,
  y: f32,
  angle: f32,
  texture: Texture2D
}

impl Base {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    let image = Image::gen_image_color(10, 10, WHITE);
    let texture = Texture2D::from_image(&image);

    Self {
      x: x,
      y: y,
      angle: angle,
      texture: texture
    }
  }

  pub fn draw(&self) {
    draw_texture(self.texture, self.x - (self.texture.width() / 2f32), self.y - (self.texture.height() / 2f32), WHITE);
  }
}