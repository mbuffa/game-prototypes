use macroquad::prelude::*;

mod game_state;
mod systems;
mod world_definition;

#[macroquad::main("Spell It")]
async fn main() {
    loop {
        clear_background(Color::from_rgba(57, 98, 233, 255));

        next_frame().await
    }
}
