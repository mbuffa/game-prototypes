use macroquad::prelude::*;

mod base;
mod gun;
mod sensor;
pub mod fire_mode;
mod rate_timer;
mod ai;

use base::Base;
use gun::Gun;
use sensor::Sensor;
use fire_mode::FireMode;
use crate::entities::monster::Monster;
use crate::assets_store::AssetsStore;

#[derive(Clone)]
pub enum GunType {
  Missile,
  Laser
}

pub struct Turret {
  identifier: String,
  base: Base,
  gun: Gun,
  sensor: Sensor,
  gun_type: GunType,
  ai: Box<dyn ai::AI>
}

impl Turret {
  pub fn new(identifier: String, x: f32, y: f32, angle: f32, gun_type: GunType, fire_mode: FireMode) -> Self {
    Self {
      identifier,
      base: Base::new(x, y),
      gun: Gun::new(x, y, 2f32, 12f32, angle, gun_type.clone(), fire_mode),
      sensor: Sensor::new(x, y, angle),
      gun_type,
      ai: Box::new(ai::BasicAI::new())
    }
  }

  pub fn draw(& self, asset_store: &AssetsStore) {
    self.base.draw(asset_store.get_texture("turret-base"));
    self.sensor.draw();
    self.gun.draw(asset_store.get_texture("turret-gun"));
  }

  pub fn update(&mut self, monsters: &mut std::slice::IterMut<'_, Monster>, dt:f32, asset_store: &AssetsStore) {
    self.ai.update(&self.base, &mut self.sensor, &mut self.gun, monsters, dt, asset_store);
  }

  pub fn is_firing(&self) -> bool { self.gun.is_firing() }

  pub fn get_gun_type(&self) -> &GunType { &self.gun_type }
  pub fn get_identifier(&self) -> &String { &self.identifier }

  pub fn get_cannon_angle(&self) -> f32 {
    self.gun.get_angle()
  }

  pub fn get_cannon_end_x(&self) -> f32 {
    self.gun.get_end_x()
  }

  pub fn get_cannon_end_y(&self) -> f32 {
    self.gun.get_end_y()
  }

  pub fn get_target_position(&self) -> Option<Vec2> {
    match &self.gun.get_target_position() {
      None => None,
      Some(position) => Some(position.clone())
    }
  }
}
