use macroquad::prelude::*;

mod missile;

pub struct Spawner {
  missiles: Vec<missile::Missile>,
  missile_texture: Texture2D
}

impl Spawner {
  pub fn new() -> Self {
    let missile_image = Image::gen_image_color(
      2, 
      20, WHITE
    );
    let missile_texture = Texture2D::from_image(&missile_image);

    Self {
      missiles: Vec::new(),
      missile_texture: missile_texture
    }
  }

  pub fn draw(&self) {
    self.missiles.iter().for_each(|missile| missile.draw(self.missile_texture));
  }

  pub fn update(&mut self, elapsed: f32) {
    self.missiles.retain(|missile| missile.rect.x >= 0f32);
    self.missiles.iter_mut().for_each(|missile| missile.update(elapsed))
  }

  pub fn spawn_missile(&mut self, x: f32, y: f32, angle: f32, velocity: f32) {
    self.missiles.push(missile::Missile::new(x, y, angle, velocity));
  }


}
