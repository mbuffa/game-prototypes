use macroquad::prelude::*;

mod base;
mod gun;
mod sensor;

use base::Base;
use gun::Gun;
use sensor::Sensor;
use crate::entities::monster::Monster;
use crate::assets_store::AssetsStore;

enum State {
  LookingForTarget,
  TargetAcquired,
  ManualOverride
}

#[derive(Clone)]
pub enum GunType {
  Missile,
  Laser
}

pub struct Turret {
  identifier: String,
  state: State,
  base: Base,
  gun: Gun,
  sensor: Sensor,
  gun_type: GunType
}

impl Turret {
  pub fn new(identifier: String, x: f32, y: f32, angle: f32, gun_type: GunType) -> Self {
    Self {
      identifier,
      state: State::LookingForTarget,
      base: Base::new(x, y),
      gun: Gun::new(x, y, 2f32, 12f32, angle, gun_type.clone()),
      sensor: Sensor::new(x, y, angle),
      gun_type: gun_type
    }
  }

  pub fn draw(& self, asset_store: &AssetsStore) {
    self.base.draw(asset_store.get_texture("turret-base"));
    self.sensor.draw();
    self.gun.draw(asset_store.get_texture("turret-gun"));
  }

  pub fn update(&mut self, monsters: &mut std::slice::IterMut<'_, Monster>, dt:f32, asset_store: &AssetsStore) {
    match self.state {
      State::LookingForTarget => {
        if is_key_pressed(KeyCode::Escape) {
          self.state = State::ManualOverride;
        }

        self.gun.update(dt, asset_store);
        self.sensor.update(dt);

        let potential_target = monsters.find(|monster| self.sees_hostile_targets(monster));

        match potential_target {
          None => {},
          Some(monster) => {
            self.acquire_target(monster.identifier(), monster.get_collider().point());
          },
        }
      },
      State::TargetAcquired => {
        self.gun.update(dt, asset_store);
        // self.sensor.update(dt);

        let target_identifier = self.gun.get_target_identifier();

        // Refresh the gun's recorded target position, or release the target if it's not in reach.
        match monsters.find(|monster| *monster.identifier() == target_identifier && monster.get_collider().y <= *self.base.y()) {
          Some(monster) => {
            self.gun.refresh_target_position(monster.get_collider().point().clone());
          },
          None => self.stand_by()
        }
      },
      State::ManualOverride => {
        if is_key_pressed(KeyCode::Escape) {
          self.stand_by();
        }

        if is_key_down(KeyCode::Left) {
          self.gun.turn_left(dt);
        }

        if is_key_down(KeyCode::Right) {
          self.gun.turn_right(dt);
        }
      }
    }
  }

  fn sees_hostile_targets(&mut self, monster: &Monster) -> bool {
    if self.sensor.sees(monster.get_collider().point()) {
      return true;
    } else {
      return false;
    }
  }

  pub fn is_firing(&self) -> bool { self.gun.is_firing() }

  pub fn acquire_target(&mut self, target_identifier: &String, target_vec: Vec2) {
    // println!("Acquiring target.");
    self.state = State::TargetAcquired;
    self.gun.acquire_target(target_identifier, target_vec);
  }

  pub fn stand_by(&mut self) {
    // println!("Standing by.");
    self.state = State::LookingForTarget;
    self.gun.release_target();
  }

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
    // self.gun.get_target()
  }
}
