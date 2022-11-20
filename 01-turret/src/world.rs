use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::assets_store::AssetsStore;

use crate::entities::turret::{Turret, GunType};
use crate::entities::laser:: Laser;
use crate::entities::missile::Missile;
use crate::entities::monster::Monster;

// const FACING_EAST: f32 = 0f32;
const FACING_NORTH: f32 = -90f32;
// const FACING_WEST: f32 = 180f32;
// const FACING_SOUTH: f32 = 90f32;

const MISSILE_VELOCITY: f32 = 200f32;
const MONSTER_VELOCITY: f32 = 40f32;

const MONSTER_SLOTS: u8 = 5;
// Monster rate of spawn, in seconds, and for each wave.
const MONSTER_SPAWN_RATE: f32 = 5f32;

const MISSILE_DAMAGE: u8 = 20;
const LASER_DAMAGE: u8 = 2;

pub struct World {
  turrets: Vec<Turret>,
  missiles: Vec<Missile>,
  lasers: HashMap<String, Laser>,
  monsters: Vec<Monster>,
  time_since_last_gen: f32,
}

impl World {
  pub fn new() -> Self {
    Self {
      turrets: Vec::new(),
      missiles: Vec::new(),
      monsters: Vec::new(),
      lasers: HashMap::new(),
      time_since_last_gen: 0f32,
    }
  }

  pub fn draw(&self, asset_store: &AssetsStore) {
    self.turrets.iter().for_each(|turret| turret.draw(asset_store));
    self.missiles.iter().for_each(|missile| missile.draw(asset_store.get_texture("missile")));
    self.monsters.iter().for_each(|monster| monster.draw(asset_store.get_texture("monster")));
    self.lasers.iter().for_each(|(k, v)| v.draw());
  }

  pub fn update(&mut self, asset_store: &AssetsStore) {
    let dt = get_frame_time();

    for turret in self.turrets.iter_mut() {
      if turret.is_firing() {
        match turret.get_gun_type() {
          GunType::Missile => {
            self.missiles.push(Missile::new(
              turret.get_cannon_end_x(),
              turret.get_cannon_end_y(),
              turret.get_cannon_angle(),
              MISSILE_VELOCITY
            ));
          },
          GunType::Laser => {
            match turret.get_target_position() {
              None => panic!("Woops!"),
              Some(position) => {
                asset_store.play_sound("fire_zzz");
                self.lasers.insert(turret.get_identifier().clone(), Laser::new(
                  turret.get_cannon_end_x(),
                  turret.get_cannon_end_y(),
                  position.x,
                  position.y
                ));
              }
            }
          }
        }
      } else {
        match turret.get_gun_type() {
          GunType::Missile => {},
          GunType::Laser => {
            self.lasers.remove(turret.get_identifier());
          }
        }
      }
      turret.update(&mut self.monsters.iter_mut(), dt, &asset_store);
    }
    
    // Missiles handling
    self.missiles.retain(|missile| missile.get_collider().x >= 0f32 && missile.is_alive());
    self.missiles.iter_mut().for_each(|missile| {      
      missile.update(dt);
    });

    // Spawns and debugging helpers for missiles, monsters, turrets.

    let pos = mouse_position();
    // if let Some(turret) = self.turrets.first() {
    //   if is_mouse_button_pressed(MouseButton::Left) {
    //     self.missiles.push(Missile::new(
    //       turret.get_cannon_end_x(),
    //       turret.get_cannon_end_y(),
    //       turret.get_cannon_angle(),
    //       MISSILE_VELOCITY
    //     ));
    //   }
    // }

    if is_mouse_button_pressed(MouseButton::Left) {
      self.turrets.push(Turret::new(
        pos.0, pos.1, FACING_NORTH, GunType::Missile
      ));
    }

    if is_mouse_button_pressed(MouseButton::Right) {
      self.turrets.push(Turret::new(
        pos.0, pos.1, FACING_NORTH, GunType::Laser
      ));
    }

    // Monsters handling
    self.maybe_spawn_monsters(dt, asset_store);

    // if is_key_pressed(KeyCode::Space) {
    //   // self.spawn_missile(pos.0, pos.1, 0f32, 0f32);
    //   self.spawn_monster(
    //     pos.0,
    //     // gen_range(200, 400) as f32,
    //     pos.1,
    //     MONSTER_VELOCITY
    //   );
    // }

    self.monsters.retain(|monster| monster.get_collider().y <= screen_height() && monster.is_alive());
    self.monsters.iter_mut().for_each(|monster| {
      self.missiles.iter_mut().for_each(|missile| {
        if monster.get_collider().overlaps(missile.get_collider()) {
          monster.hit(MISSILE_DAMAGE);
          missile.destroy();
        }
      });

      self.lasers.iter().for_each(|(_identifier, laser)| {
        if monster.get_collider().contains(laser.get_target_position()) {
          monster.hit(LASER_DAMAGE);
        }
      });

      monster.update(dt);

      // println!("Lasers: {}", self.lasers.len());
    });
  }

  // pub fn spawn_missile(&mut self, x: f32, y: f32, angle: f32, velocity: f32) {
  //   self.missiles.push(Missile::new(x, y, angle, velocity));
  // }

  pub fn spawn_monster(&mut self, x: f32, y: f32, velocity: f32) {
    self.monsters.push(Monster::new(x, y, velocity));
  }

  fn maybe_spawn_monsters(&mut self, dt: f32, asset_store: &AssetsStore) {
    let texture = asset_store.get_texture("monster");

    if self.time_since_last_gen >= MONSTER_SPAWN_RATE {
      let to_generate = gen_range(1, MONSTER_SLOTS);
      let slot_length: f32 = screen_width() / to_generate as f32;

      // println!("To gen: {}", to_generate);

      for i in 1..to_generate {
        self.spawn_monster(
          i as f32 * slot_length - (texture.width()) * 0.5f32,
          -texture.height(),
          MONSTER_VELOCITY
        );
      }
      self.time_since_last_gen = 0f32;
    } else {
      self.time_since_last_gen += dt;
    }
  }

}