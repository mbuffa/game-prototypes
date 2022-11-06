use macroquad::prelude::*;

// Rotation direction. -1 means it'll initially go to the left.
const ROT_DIRECTION: f32 = -1f32;

// Rotation velocity, in degrees per second.
const ROT_VELOCITY: f32 = 30f32;

// Left boundary, in degrees, relative to base angle.
const LEFT_BOUNDARY: f32 = -60f32;

// Right boundary, in degrees, relative to base angle.
const RIGHT_BOUNDARY: f32 = 60f32;

pub struct Gun {
  x: f32,
  y: f32,
  base_angle: f32,
  angle: f32,
  texture: Texture2D,
  rot_direction: f32
}

impl Gun {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    let image = Image::gen_image_color(2, 12, WHITE);
    let texture = Texture2D::from_image(&image);

    Self {
      x: x,
      y: y,
      base_angle: angle,
      angle: angle,
      texture: texture,
      rot_direction: ROT_DIRECTION
    }
  }

  pub fn draw(&self) {
    draw_texture_ex(
      self.texture,
      self.get_gun_end_x(),
      self.get_gun_end_y(),
      WHITE,
      DrawTextureParams { rotation: (self.angle + 90f32).to_radians(), dest_size: None, source: None, flip_x: false, flip_y: false, pivot: Some(Vec2::new(self.x, self.y)) }
    );
  }

  pub fn update(&mut self, elapsed: f32) {
    if self.angle <= self.base_angle + LEFT_BOUNDARY {
      self.rot_direction = 1f32;
    }

    if self.angle >= self.base_angle + RIGHT_BOUNDARY {
      self.rot_direction = -1f32;
    }

    self.angle += self.rot_direction * ROT_VELOCITY * elapsed;
  }

  pub fn get_gun_end_x(&self) -> f32 {
    self.x - (self.texture.width() / 2f32)
  }

  pub fn get_gun_end_y(&self) -> f32 {
    self.y - (self.texture.height())
  }

  pub fn get_angle(&self) -> f32 { self.angle }
}