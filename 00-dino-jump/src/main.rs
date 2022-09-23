use macroquad::{prelude::*};

mod game;

#[macroquad::main("BasicShapes")]
async fn main() {
  let mut context = game::Context::new();
  
  loop {
    context.update();
    clear_background(WHITE);
    context.draw();
    next_frame().await
  }
}