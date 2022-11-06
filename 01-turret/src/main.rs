use macroquad::prelude::*;

mod entities;
mod world;
use world::World;

#[macroquad::main("Turret")]
async fn main() {
  let mut world = World::new();

  loop {
    world.update();
    clear_background(BLACK);
    world.draw();
    next_frame().await
  }
  
}
