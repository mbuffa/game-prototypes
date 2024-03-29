use macroquad::prelude::*;

use crate::assets_store::AssetsStore;

use super::GunType;
use super::fire_mode::FireMode;
use super::rate_timer::RateTimer;

// Rotation direction. -1 means it'll initially go to the left.
const ROT_DIRECTION: f32 = -1f32;

// Rotation velocity, in degrees per second.
const ROT_VELOCITY: f32 = 45f32;

pub struct Gun {
  x: f32,
  y: f32,
  height: f32,
  angle: f32,
  rot_direction: f32,
  target: Option<(String, Vec2)>,
  is_firing: bool,
  gun_type: GunType,
  rate_timer: RateTimer
}

impl Gun {
  pub fn new(x: f32, y: f32, _width: f32, height: f32, angle: f32, gun_type: GunType, fire_mode: FireMode) -> Self {
    Self {
      x,
      y,
      height,
      angle,
      rot_direction: ROT_DIRECTION,
      target: None,
      is_firing: false,
      gun_type,
      rate_timer: RateTimer::new(fire_mode)
    }
  }

  pub fn draw(&self, texture: &Texture2D) {
    draw_texture_ex(
      *texture,
      self.get_base_end_x() - ((*texture).width() / 2f32),
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

  pub fn update(&mut self, elapsed: f32, asset_store: &AssetsStore) {
    self.rate_timer.update(elapsed);

    match &self.target {
      None => {
        // self.time_since_last_shot += elapsed;
        self.rate_timer.increment(elapsed);
      },
      Some((_target_identifier, target_vec)) => {
        let angle_to_target = Vec2::angle_between(
          Vec2::new(self.x - self.get_end_x(), self.y - self.get_end_y()),
          Vec2::new(self.x - target_vec.x, self.y - target_vec.y)
        ).to_degrees();

        if angle_to_target.abs() < 0.010f32 {
          self.maybe_fire(elapsed, asset_store);
        } else {
          self.is_firing = false;
          self.rate_timer.increment(elapsed);

          let mut rot_velocity = ROT_VELOCITY * elapsed;

          if angle_to_target.abs() - rot_velocity <= 0f32 {
            rot_velocity = angle_to_target;

            self.maybe_fire(elapsed, asset_store);
          }

          if angle_to_target <= 0f32 {
            self.rot_direction = -1f32;
          }
          if angle_to_target >= 0f32 {
            self.rot_direction = 1f32;
          }

          self.angle += self.rot_direction * rot_velocity;
          // println!("target: {}, angle: {}", angle_to_target.abs(), self.angle);
        }
      }
    }
  }

  pub fn acquire_target(&mut self, target_identifier: &String, target_vec: Vec2) {
    // println!("Acquiring target {} {}", target_identifier, target_vec);
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

  pub fn get_target_position(&self) -> Option<Vec2> {
    match &self.target {
      None => None,
      Some((_target_identifier, position)) => {
        Some(position.clone())
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

  fn maybe_fire(&mut self, dt: f32, asset_store: &AssetsStore) {
    match self.gun_type {
      GunType::Missile => {
        if self.rate_timer.can_shoot() {
          self.rate_timer.reset();
          self.is_firing = true;
          asset_store.play_sound("fire");
        } else {
          self.rate_timer.increment(dt);
          self.is_firing = false;
        }
      },
      GunType::Laser => {
        self.is_firing = true;
      }
    }
  }

  fn get_base_end_x(&self) -> f32 {
    self.x
  }

  fn get_base_end_y(&self) -> f32 {
    self.y
  }

  pub fn get_end_x(&self) -> f32 {
    self.x + (self.height) * (self.angle).to_radians().cos()
  }

  pub fn get_end_y(&self) -> f32 {
    self.y + (self.height) * (self.angle).to_radians().sin()
  }

  pub fn get_angle(&self) -> f32 { self.angle }
}