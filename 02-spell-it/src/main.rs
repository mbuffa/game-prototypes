use game_master::GameMaster;
use macroquad::prelude::*;

mod game_state;
mod game_master;
mod rendering;
mod systems;
mod utils;
mod world_definition;

use game_state::GameState;
use world_definition::WorldDefinition;

#[macroquad::main("Spell It")]
async fn main() {
    let mut game_master = GameMaster::new();
    let mut since_last_frame: f32;

    game_master.start_session();

    loop {
        since_last_frame = macroquad::time::get_frame_time();

        let (definition, game_state, input) =
            game_master.update(&since_last_frame);

        // clear_background(Color::from_rgba(57, 98, 233, 255));
        clear_background(BROWN);

        draw(&definition, &game_state, &input);

        next_frame().await
    }
}

fn draw(definition: &WorldDefinition, game_state: &GameState, input: &String) {
    rendering::draw_scene(&definition, &game_state, &input);
    rendering::draw_ui(&definition, &game_state, &input);
    // rendering::draw_ui_debug(&game_state);
}
