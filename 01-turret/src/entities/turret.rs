use macroquad::prelude::*;

mod base;
mod gun;
mod sensor;

use base::Base;
use gun::Gun;
use sensor::Sensor;
use crate::entities::monster::Monster;

enum State {
  LookingForTarget,
  TargetAcquired,
  ManualOverride
}

pub struct Turret {
  state: State,
  base: Base,
  gun: Gun,
  sensor: Sensor
}

impl Turret {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    Self {
      state: State::LookingForTarget,
      base: Base::new(x, y),
      gun: Gun::new(x, y, angle),
      sensor: Sensor::new(x, y, angle)
    }
  }

  pub fn draw(& self) {
    self.base.draw();
    self.sensor.draw();
    self.gun.draw();
  }

  pub fn update(&mut self, monsters: &mut std::slice::IterMut<'_, Monster>, dt:f32) {
    match self.state {
      State::LookingForTarget => {
        if is_key_pressed(KeyCode::Escape) {
          self.state = State::ManualOverride;
        }

        self.gun.update(dt);
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
        self.gun.update(dt);

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
    println!("Acquiring target.");
    self.state = State::TargetAcquired;
    self.gun.acquire_target(target_identifier, target_vec);
  }

  pub fn stand_by(&mut self) {
    println!("Standing by.");
    self.state = State::LookingForTarget;
    self.gun.release_target();
  }

  pub fn get_cannon_angle(&self) -> f32 {
    self.gun.get_angle()
  }

  pub fn get_cannon_end_x(&self) -> f32 {
    self.gun.get_end_x()
  }

  pub fn get_cannon_end_y(&self) -> f32 {
    self.gun.get_end_y()
  }
}
