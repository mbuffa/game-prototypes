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
    clear_background(BLACK);
    world.draw(&store);
    next_frame().await
  }
  
}
