use macroquad::{prelude::*};

mod turret;

const FACING_EAST: f32 = 0f32;
const FACING_NORTH: f32 = -90f32;
const FACING_WEST: f32 = -180f32;
const FACING_SOUTH: f32 = 90f32;

#[macroquad::main("Turret")]
async fn main() {
  let x = screen_width() / 2f32;
  let y = screen_height() / 2f32;
  let angle = FACING_WEST;

  let mut turret = turret::Turret::new(x, y, angle);
  
  loop {
    turret.update();
    clear_background(BLACK);
    turret.draw();
    next_frame().await
  }
  
}
