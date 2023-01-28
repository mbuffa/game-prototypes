use macroquad::prelude::*;

mod entities;
mod assets_store;
mod world;

use world::World;
use assets_store::AssetsStore;

#[macroquad::main("Turret")]
async fn main() {
  let mut world = World::new();
  let mut store: AssetsStore = AssetsStore::new();
  store.load_assets().await;

  loop {
    world.update(&store);

    clear_background(
      Color::from_rgba(57, 98, 233, 255)
    );
    
    world.draw(&store);
    next_frame().await
  }
  
}
