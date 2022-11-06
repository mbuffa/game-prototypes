use macroquad::prelude::*;

use crate::entities::turret::Turret;
use crate::entities::missile::Missile;
use crate::entities::monster::Monster;

// const FACING_EAST: f32 = 0f32;
// const FACING_NORTH: f32 = -90f32;
const FACING_WEST: f32 = 180f32;
// const FACING_SOUTH: f32 = 90f32;

const MISSILE_VELOCITY: f32 = 200f32;
// const MONSTER_VELOCITY: f32 = 80f32;
// Debug:
const MONSTER_VELOCITY: f32 = 0f32;
pub struct World {
  turrets: Vec<Turret>,
  missiles: Vec<Missile>,
  monsters: Vec<Monster>,
  missile_texture: Texture2D,
  monster_texture: Texture2D
}

impl World {
  pub fn new() -> Self {
    let turret = Turret::new(screen_width() / 2f32, screen_height() / 2f32, FACING_WEST);
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
      monster_texture
    }
  }

  pub fn draw(&self) {
    self.turrets.iter().for_each(|turret| turret.draw());
    self.missiles.iter().for_each(|missile| missile.draw(&self.missile_texture));
    self.monsters.iter().for_each(|monster| monster.draw(self.monster_texture));
  }

  pub fn update(&mut self) {
    let dt = get_frame_time();

    for turret in self.turrets.iter_mut() {
      // TODO: Use a timer to limit rate of fire.
      if turret.is_firing() {
        self.missiles.push(Missile::new(turret.get_cannon_end_x(), turret.get_cannon_end_y(), turret.get_cannon_angle(), MISSILE_VELOCITY));
      }
      turret.update(&mut self.monsters.iter_mut(), dt);
    }
    
    // Missiles handling
    self.missiles.retain(|missile| missile.get_collider().x >= 0f32 && missile.is_alive());
    self.missiles.iter_mut().for_each(|missile| {      
      missile.update(dt);
    });
    if let Some(turret) = self.turrets.first() {
      if is_mouse_button_pressed(MouseButton::Left) {
        self.spawn_missile(
          turret.get_cannon_end_x(),
          turret.get_cannon_end_y(),
          turret.get_cannon_angle(),
          MISSILE_VELOCITY
        );
      }
    }

    // Monsters handling
    if is_key_pressed(KeyCode::Space) {
      let pos = mouse_position();
      // self.spawn_missile(pos.0, pos.1, 0f32, 0f32);

      self.spawn_monster(
        pos.0,
        // gen_range(200, 400) as f32,
        pos.1,
        MONSTER_VELOCITY
      );
    }

    self.monsters.retain(|monster| monster.get_collider().x >= 0f32 && monster.is_alive());
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

  pub fn spawn_missile(&mut self, x: f32, y: f32, angle: f32, velocity: f32) {
    self.missiles.push(Missile::new(x, y, angle, velocity));
  }

  pub fn spawn_monster(&mut self, x: f32, y: f32, velocity: f32) {
    self.monsters.push(Monster::new(x, y, velocity));
  }
}