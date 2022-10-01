
use std::vec;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

use super::dino::Dino;
use super::obstacle::Obstacle;

pub struct Spawner {
  obstacles: Vec<Obstacle>,
  idle_for: f32
}

impl Spawner {
  pub fn new() -> Self {
    Self {
      obstacles: vec!(),
      idle_for: get_frame_time()
    }
  }
  
  pub fn reset(&mut self) {
    self.obstacles =  vec!();
    self.idle_for = get_frame_time();
  }
  
  pub fn update(&mut self, elapsed: f32) -> bool {
    let all_obstacles = self.obstacles.to_owned();
    
    self.obstacles.retain(|obstacle| obstacle.rect.x >= 0f32);
    let remaining_obstacles = self.obstacles.to_owned();
    let avoided_obstacles: Vec<&Obstacle> =
    all_obstacles
    .iter()
    .filter(|obstacle| remaining_obstacles.iter().all(|remaining| remaining.rect.x != obstacle.rect.x))
    .collect();
    
    avoided_obstacles.iter().for_each(|obs| println!("Avoided: {}", obs));
    
    self.obstacles.iter_mut().for_each(|obstacle| obstacle.update(elapsed));
    
    // Generate a second obstacle.
    if self.idle_for >= 0.1f32 && self.idle_for < 0.11f32 {
      if gen_range(0, 10) * 10 < 20 {
        self.generate();
      }
    }
    
    // Generate a third obstacle.
    if self.idle_for >= 0.2f32 && self.idle_for < 0.21f32 {
      if gen_range(0, 10) * 10 < 20 {
        self.generate();
      }
    }
    
    if self.idle_for >= 2f32 {
      self.generate();
      self.idle_for = 0f32;
    } else {
      self.idle_for += elapsed;
    }
    
    avoided_obstacles.len() == 1
  }
  
  pub fn generate(&mut self) {
    self.obstacles.push(Obstacle::new());
  }
  
  pub fn any_child_collided(& self, dino: &Dino) -> bool {
    self.obstacles.iter().any(|obstacle: &Obstacle|
      match obstacle.rect.intersect(dino.rect) {
        Some(_intersection) => true,
        _ => false
      }
    )
  }
  
  pub fn draw_obstacles(& self) {
    self.obstacles.iter().for_each(|obstacle| obstacle.draw());
  }
}
