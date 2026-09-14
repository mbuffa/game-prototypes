use macroquad::prelude::*;

mod assets_store;
mod entities;
mod world;

use assets_store::AssetsStore;
use world::World;

#[macroquad::main("Turret")]
async fn main() {
    let mut world = World::new();
    let mut store: AssetsStore = AssetsStore::new();
    store.load_assets().await;

    loop {
        world.update(&store);

        clear_background(Color::from_rgba(57, 98, 233, 255));

        world.draw(&store);
        next_frame().await
    }
}
