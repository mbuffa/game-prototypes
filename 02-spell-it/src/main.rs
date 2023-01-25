use game_master::GameMaster;
use macroquad::prelude::*;

mod game_master;
mod game_state;
mod rendering;
mod systems;
mod utils;
mod world;

use game_state::GameState;
use world::World;

#[macroquad::main("Spell It")]
async fn main() {
    let mut game_master = GameMaster::new();
    let mut since_last_frame: f32;

    game_master.start_session();

    loop {
        since_last_frame = macroquad::time::get_frame_time();

        let (world, game_state, input) = game_master.update(&since_last_frame);

        draw(&world, &game_state, &input);

        next_frame().await
    }
}

fn draw(world: &World, game_state: &GameState, input: &String) {
    if game_state.is_won() {
        rendering::draw_victory_screen();
    } else if game_state.is_over() {
        rendering::draw_defeat_screen();
    } else {
        rendering::draw_game_screen(&world, &game_state, &input);
    }

    // rendering::draw_ui_debug(&game_state);
}
