use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::audio::Sound;

pub struct AssetsStore {
  textures: HashMap<String, Texture2D>,
  sounds: HashMap<String, Sound>
}

impl AssetsStore {
  pub fn new() -> Self {
    let missile_image = Image::gen_image_color(4, 4, ORANGE);
    let missile_texture = Texture2D::from_image(&missile_image);

    let monster_image = Image::gen_image_color(40, 40, RED);
    let monster_texture = Texture2D::from_image(&monster_image);

    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    textures.insert("missile".to_owned(), missile_texture);
    textures.insert("monster".to_owned(), monster_texture);

    AssetsStore {
      textures: textures,
      sounds: HashMap::new()
    }
  }

  pub async fn load_assets(&mut self) {
    let fire = macroquad::audio::load_sound("assets/pew.wav").await.expect("Failed lkoading 'pew'");
    self.sounds.insert("fire".to_owned(), fire);
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
        macroquad::audio::play_sound_once(*sound);
      },
      None => {}
    }
  }
}