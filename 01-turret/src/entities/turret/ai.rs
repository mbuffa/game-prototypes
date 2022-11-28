use macroquad::prelude::*;

use crate::AssetsStore;
use super::{Base, Gun, Sensor};
use crate::entities::monster::Monster;

enum State {
  LookingForTarget,
  TargetAcquired,
  ManualOverride
}

pub trait AI {
  fn update(&mut self, base: &Base, sensor: &mut Sensor, gun: &mut Gun, monsters: &mut std::slice::IterMut<'_, Monster>, dt: f32, asset_store: &AssetsStore);
}

pub struct BasicAI {
  state: State
}

impl AI for BasicAI {
  fn update(&mut self, base: &Base, sensor: &mut Sensor, gun: &mut Gun, monsters: &mut std::slice::IterMut<'_, Monster>, dt: f32, asset_store: &AssetsStore) {
    match self.state {
      State::LookingForTarget => {
        if is_key_pressed(KeyCode::Escape) {
          self.state = State::ManualOverride;
        }

        gun.update(dt, asset_store);
        sensor.update(dt);

        let potential_target = monsters.find(|monster| self.sees_hostile_targets(sensor, monster));

        match potential_target {
          None => {},
          Some(monster) => {
            asset_store.play_sound("spotted");
            self.acquire_target(gun, monster.identifier(), monster.get_collider().point());
          },
        }
      },
      State::TargetAcquired => {
        gun.update(dt, asset_store);
        // self.sensor.update(dt);

        let target_identifier = gun.get_target_identifier();

        // Refresh the gun's recorded target position, or release the target if it's not in reach.
        match monsters.find(|monster| *monster.identifier() == target_identifier && monster.get_collider().y <= *base.y()) {
          Some(monster) => {
            gun.refresh_target_position(monster.get_collider().point().clone());
          },
          None => self.stand_by(gun)
        }
      },
      State::ManualOverride => {
        if is_key_pressed(KeyCode::Escape) {
          self.stand_by(gun);
        }

        if is_key_down(KeyCode::Left) {
          gun.turn_left(dt);
        }

        if is_key_down(KeyCode::Right) {
          gun.turn_right(dt);
        }
      }
    }
  }
}

impl BasicAI {
  pub fn new() -> Self {
    BasicAI {
      state: State::LookingForTarget
    }
  }

  fn sees_hostile_targets(&mut self, sensor: &Sensor, monster: &Monster) -> bool {
    if sensor.sees(monster.get_collider().point()) {
      return true;
    } else {
      return false;
    }
  }

  fn acquire_target(&mut self, gun: &mut Gun, target_identifier: &String, target_vec: Vec2) {
    println!("Acquiring target.");
    self.state = State::TargetAcquired;
    gun.acquire_target(target_identifier, target_vec);
  }

  fn stand_by(&mut self, gun: &mut Gun) {
    println!("Standing by.");
    self.state = State::LookingForTarget;
    gun.release_target();
  }
}

