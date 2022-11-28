use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::audio::Sound;

pub struct AssetsStore {
  textures: HashMap<String, Texture2D>,
  sounds: HashMap<String, Sound>
}

impl AssetsStore {
  pub fn new() -> Self {
    let missile_image = Image::gen_image_color(4, 4, WHITE);
    let missile_texture = Texture2D::from_image(&missile_image);

    let monster_image = Image::gen_image_color(40, 40, WHITE);
    let monster_texture = Texture2D::from_image(&monster_image);

    let turret_base_image = Image::gen_image_color(10, 10, WHITE);
    let turret_base_texture = Texture2D::from_image(&turret_base_image);

    let turret_gun_image = Image::gen_image_color(2, 12, WHITE);
    let turret_gun_texture = Texture2D::from_image(&turret_gun_image);

    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    textures.insert("missile".to_owned(), missile_texture);
    textures.insert("monster".to_owned(), monster_texture);
    textures.insert("turret-base".to_owned(), turret_base_texture);
    textures.insert("turret-gun".to_owned(), turret_gun_texture);

    AssetsStore {
      textures,
      sounds: HashMap::new()
    }
  }

  pub async fn load_assets(&mut self) {
    let fire = macroquad::audio::load_sound("assets/tff.wav").await.expect("Failed lkoading 'pew'");
    self.sounds.insert("fire".to_owned(), fire);

    let laser = macroquad::audio::load_sound("assets/fff.wav").await.expect("Failed lkoading 'zzz'");
    self.sounds.insert("laser".to_owned(), laser);

    let death = macroquad::audio::load_sound("assets/argh.wav").await.expect("Failed lkoading 'pew'");
    self.sounds.insert("death".to_owned(), death);

    let impact = macroquad::audio::load_sound("assets/poof.wav").await.expect("Failed lkoading 'pew'");
    self.sounds.insert("impact".to_owned(), impact);

    let spotted = macroquad::audio::load_sound("assets/spotted.wav").await.expect("Failed lkoading 'pew'");
    self.sounds.insert("spotted".to_owned(), spotted);
  }

  pub fn get_texture(&self, name: &str) -> &Texture2D {
    match self.textures.get(name) {
      Some(texture) => texture,
      None => panic!("Couldn't find texture {}", name)
    }
  }

  pub fn play_sound(&self, name: &str) {
    match self.sounds.get(name) {
      Some(sound) => {
        // macroquad::audio::play_sound_once(*sound);
      },
      None => {}
    }
  }
}