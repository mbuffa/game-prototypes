use std::collections::HashMap;

use macroquad::audio::Sound;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::entities::turret::Turret;
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

pub struct World {
  turrets: Vec<Turret>,
  missiles: Vec<Missile>,
  monsters: Vec<Monster>,
  missile_texture: Texture2D,
  monster_texture: Texture2D,
  time_since_last_gen: f32,
  soundbank: HashMap<String, Sound>
}

impl World {
  pub fn new() -> Self {
    let turret = Turret::new(screen_width() / 2f32, screen_height() / 2f32, FACING_NORTH);
    let mut turrets = Vec::new();
    turrets.push(turret);

    let missile_image = Image::gen_image_color(4, 4, ORANGE);
    let missile_texture = Texture2D::from_image(&missile_image);

    let monster_image = Image::gen_image_color(40, 40, RED);
    let monster_texture = Texture2D::from_image(&monster_image);

    Self {
      turrets,
      missiles: Vec::new(),
      monsters: Vec::new(),
      missile_texture,
      monster_texture,
      time_since_last_gen: 0f32,
      soundbank: HashMap::new()
    }
  }

  pub async fn load_assets(&mut self) {
    let fire = macroquad::audio::load_sound("assets/pew.wav").await.expect("Failed lkoading 'pew'");
    self.soundbank.insert("fire".to_owned(), fire);
  }

  pub fn draw(&self) {
    self.turrets.iter().for_each(|turret| turret.draw());
    self.missiles.iter().for_each(|missile| missile.draw(&self.missile_texture));
    self.monsters.iter().for_each(|monster| monster.draw(self.monster_texture));
  }

  pub fn update(&mut self) {
    let dt = get_frame_time();

    for turret in self.turrets.iter_mut() {
      if turret.is_firing() {
        self.missiles.push(Missile::new(turret.get_cannon_end_x(), turret.get_cannon_end_y(), turret.get_cannon_angle(), MISSILE_VELOCITY));
        Self::play_sound(&self.soundbank, "fire");
      }
      turret.update(&mut self.monsters.iter_mut(), dt);
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
    //     self.spawn_missile(
    //       turret.get_cannon_end_x(),
    //       turret.get_cannon_end_y(),
    //       turret.get_cannon_angle(),
    //       MISSILE_VELOCITY
    //     );
    //   }
    // }

    if is_mouse_button_pressed(MouseButton::Right) {
      self.turrets.push(Turret::new(
        pos.0, pos.1, FACING_NORTH
      ));
    }

    // Monsters handling
    self.maybe_spawn_monsters(dt);

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
          monster.hit();
          missile.destroy();
        }
      });

      monster.update(dt);
    });
  }

  // pub fn spawn_missile(&mut self, x: f32, y: f32, angle: f32, velocity: f32) {
  //   self.missiles.push(Missile::new(x, y, angle, velocity));
  // }

  pub fn spawn_monster(&mut self, x: f32, y: f32, velocity: f32) {
    self.monsters.push(Monster::new(x, y, velocity));
  }

  fn maybe_spawn_monsters(&mut self, dt: f32) {
    if self.time_since_last_gen >= MONSTER_SPAWN_RATE {
      let total_slots = screen_width() / self.monster_texture.width();
      let to_generate = gen_range(1, MONSTER_SLOTS);
      println!("To gen: {}", to_generate);
      let spacing = total_slots / to_generate as f32 * self.monster_texture.width();

      for i in 2..(to_generate + 2) {
        self.spawn_monster(
          i as f32 * spacing,
          -self.monster_texture.height(),
          MONSTER_VELOCITY
        );
      }
      self.time_since_last_gen = 0f32;
    } else {
      self.time_since_last_gen += dt;
    }
  }

  fn play_sound(bank: &HashMap<String, Sound>, name: &str) {
    match bank.get(name) {
      Some(sound) => {
        macroquad::audio::play_sound_once(*sound);
      },
      None => {}
    }
  }
}