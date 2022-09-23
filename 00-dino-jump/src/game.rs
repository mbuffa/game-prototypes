use macroquad::{prelude::*};

mod actors;
use actors::dino::Dino;
use actors::spawner::Spawner;

enum State {
  Playing,
  GameOver
}

pub struct Context {
  state: State,
  score: u32,
  dino: Dino,
  spawner: Spawner,
  timer: f32
}

impl Context {
  pub fn new() -> Self {
    Self {
      state: State::Playing,
      score: 0,
      dino: Dino::new(),
      spawner: Spawner::new(),
      timer: 0f32
    }
  }
  
  pub fn reset(&mut self) {
    self.score = 0;
    self.state = State::Playing;
    self.spawner.reset();
  }
  
  pub fn game_over(&mut self) {
    self.state = State::GameOver;
  }
  
  pub fn draw(& self) {
    self.dino.draw();
    self.spawner.draw_obstacles();
    
    match self.state {
      State::Playing => {
        let score_text = std::fmt::format(format_args!("Score: {}", self.score));
        draw_text(&score_text, 10f32, 48f32, 48f32, BLACK);
      },
      _ => {}
    }
  }
  
  pub fn update(&mut self) {
    let dt = get_frame_time();
    
    self.timer += dt;
    
    match self.state {
      State::Playing => {
        
        self.dino.update(dt);
        
        // Collision
        if self.spawner.any_child_collided(& self.dino) {
          self.game_over();
        }
        
        let has_avoided = self.spawner.update(dt);
        
        if has_avoided {
          self.score += 10;
        }
      },
      _ => {
        if is_key_pressed(KeyCode::Space) {
          self.reset();
        }
      }
    }
  }
}
