use macroquad::prelude::*;

// Rotation direction. -1 means it'll initially go to the left.
const ROT_DIRECTION: f32 = -1f32;

// Rotation velocity, in degrees per second.
const ROT_VELOCITY: f32 = 18f32;

// Left boundary, in degrees, relative to base angle.
const LEFT_BOUNDARY: f32 = -60f32;

// Right boundary, in degrees, relative to base angle.
const RIGHT_BOUNDARY: f32 = 60f32;

// Rate of fire, in seconds.
const RATE_OF_FIRE: f32 = 1f32;

pub struct Gun {
  x: f32,
  y: f32,
  base_angle: f32,
  angle: f32,
  texture: Texture2D,
  rot_direction: f32,
  target: Option<(String, Vec2)>,
  is_firing: bool,
  time_since_last_shot: f32
}

impl Gun {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    let image = Image::gen_image_color(2, 12, WHITE);
    let texture = Texture2D::from_image(&image);

    Self {
      x,
      y,
      base_angle: angle,
      angle,
      texture,
      rot_direction: ROT_DIRECTION,
      target: None,
      is_firing: false,
      time_since_last_shot: 0f32
    }
  }

  pub fn draw(&self) {
    draw_texture_ex(
      self.texture,
      self.get_base_end_x() - (self.texture.width() / 2f32),
      self.get_base_end_y() ,
      WHITE,
      DrawTextureParams { rotation: (self.angle - 90f32).to_radians(), dest_size: None, source: None, flip_x: false, flip_y: false, pivot: Some(Vec2::new(self.x, self.y)) }
    );

    // Debug: actual "gun" used to determine missile starting position.
    // draw_line(
    //   self.get_base_end_x(),
    //   self.get_base_end_y(),
    //   self.get_end_x(),
    //   self.get_end_y(),
    //   10f32,
    //   BLUE
    // );
  }

  pub fn update(&mut self, elapsed: f32) {
    match &self.target {
      None => {
        if self.angle <= self.base_angle + LEFT_BOUNDARY {
          self.rot_direction = 1f32;
        }
    
        if self.angle >= self.base_angle + RIGHT_BOUNDARY {
          self.rot_direction = -1f32;
        }
    
        self.angle += self.rot_direction * ROT_VELOCITY * elapsed;
        self.time_since_last_shot += elapsed;
      },
      Some((_target_identifier, target_vec)) => {
        let angle_to_target = Vec2::angle_between(
          Vec2::new(self.x - self.get_end_x(), self.y - self.get_end_y()),
          Vec2::new(self.x - target_vec.x, self.y - target_vec.y)
        ).to_degrees();

        if angle_to_target.abs() < 0.010f32 {
          self.maybe_fire(elapsed);
        } else {
          let mut rot_velocity = ROT_VELOCITY * elapsed;

          if angle_to_target.abs() - rot_velocity <= 0f32 {
            rot_velocity = angle_to_target;

            self.maybe_fire(elapsed);
          }

          if angle_to_target <= 0f32 {
            self.rot_direction = -1f32;
          }
          if angle_to_target >= 0f32 {
            self.rot_direction = 1f32;
          }

          self.angle += self.rot_direction * rot_velocity;
          self.time_since_last_shot += elapsed;
          // println!("target: {}, angle: {}", angle_to_target.abs(), self.angle);
        }
      }
    }
  }

  pub fn acquire_target(&mut self, target_identifier: &String, target_vec: Vec2) {
    println!("Acquiring target {} {}", target_identifier, target_vec);
    self.target = Some((target_identifier.clone(), target_vec));
  }

  pub fn release_target(&mut self) {
    self.target = None;
    self.is_firing = false;
  }

  pub fn refresh_target_position(&mut self, new_target_vec: Vec2) {
    match &self.target {
      None => {},
      Some((identifier, _old_target_vec)) => {
        self.target = Some((identifier.clone(), new_target_vec));
      }
    }
  }

  pub fn get_target_identifier(&self) -> String {
    match &self.target {
      None => panic!("Gun target is null!"),
      Some((target_identifier, _)) => {
        target_identifier.clone()
      }
    }
  }

  pub fn turn_left(&mut self, dt: f32) {
    self.rot_direction = -1f32;
    self.angle += self.rot_direction * ROT_VELOCITY * dt;
  }

  pub fn turn_right(&mut self, dt: f32) {
    self.rot_direction = 1f32;
    self.angle += self.rot_direction * ROT_VELOCITY * dt;
  }

  pub fn is_firing(&self) -> bool { self.is_firing }

  fn maybe_fire(&mut self, dt: f32) {
    if self.time_since_last_shot >= RATE_OF_FIRE {
      self.is_firing = true;
      self.time_since_last_shot = 0f32;
    } else {
      self.is_firing = false;
      self.time_since_last_shot += dt;
    }
  }

  fn get_base_end_x(&self) -> f32 {
    self.x
  }

  fn get_base_end_y(&self) -> f32 {
    self.y
  }

  pub fn get_end_x(&self) -> f32 {
    self.x + (self.texture.height() + 10f32) * (self.angle).to_radians().cos()
  }

  pub fn get_end_y(&self) -> f32 {
    self.y + (self.texture.height() + 10f32) * (self.angle).to_radians().sin()
  }

  pub fn get_angle(&self) -> f32 { self.angle }
}