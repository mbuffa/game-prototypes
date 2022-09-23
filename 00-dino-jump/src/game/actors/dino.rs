use macroquad::prelude::*;

pub struct Dino {
  pub rect: Rect,
  is_jumping: bool,
  jumped_at: Option<f32>,
  
}

impl Dino {
  pub fn new() -> Self {
    let (width, height) = (50f32, 120f32);
    Self {
      rect: Rect::new(width, (screen_height() - height) * 0.5f32, width, height),
      is_jumping: false,
      jumped_at: None
    }
  }
  
  pub fn update(&mut self, elapsed: f32) {
    if self.is_jumping == false && is_key_pressed(KeyCode::Space) {
      self.is_jumping = true;
    }
    
    if self.is_jumping {
      self.rect.y = (screen_height() - 180f32 - 120f32) * 0.5f32;
      
      self.jumped_at = match self.jumped_at {
        None =>
        Some(elapsed),
        
        Some(delta_time) =>
        if delta_time >= 0.6f32 {
          self.is_jumping = false;
          self.rect.y = (screen_height() - 120f32) * 0.5f32;
          None
        } else {
          // TODO: Use cos() or sin() to get a smooth jump.
          if delta_time <= 0.4f32 {
            self.rect.y -= 10f32;
          }
          
          if delta_time > 0.4f32 && delta_time < 0.6f32 {
            self.rect.y += 20f32;
          }
          
          Some(delta_time + elapsed)
        }
      }
    }
  }
  
  pub fn draw(& self) {
    draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK)
  }
}