use macroquad::prelude::*;

mod game_state;
mod rendering;
mod systems;
mod utils;
mod world_definition;

use game_state::GameState;
use world_definition::WorldDefinition;

#[macroquad::main("Spell It")]
async fn main() {
    let mut definition = WorldDefinition::new();
    let mut game_state = GameState::new();
    let mut input: String = String::from("");

    let mut since_last_frame: f32;
    let mut since_last_stroke: f64;

    definition.initialize_spell_types();
    game_state.initialize();

    loop {
        since_last_frame = macroquad::time::get_frame_time();
        since_last_stroke = macroquad::time::get_time();

        systems::maybe_go_to_next_stage(&mut game_state);
        systems::maybe_mark_game_as_over_or_won(&mut game_state);

        if game_state.is_won() {
            println!("You won!");
        }

        (definition, game_state, input) = update(
            definition,
            game_state,
            input,
            &since_last_frame,
            &mut since_last_stroke,
        );

        // clear_background(Color::from_rgba(57, 98, 233, 255));
        clear_background(BROWN);

        draw(&definition, &game_state, &input);

        next_frame().await
    }
}

fn update(
    definition: WorldDefinition,
    mut game_state: GameState,
    input: String,
    _since_last_frame: &f32,
    _since_last_stroke: &mut f64,
) -> (WorldDefinition, GameState, String) {
    if is_key_pressed(KeyCode::Enter) {
        let sanitized = systems::sanitize_input(input);

        if systems::validate_input(&definition, &sanitized) {
            match systems::execute_input(&definition, &mut game_state, &sanitized) {
                _ => debug!("Updated Scene."),
            }

            (definition, game_state, String::new())
        } else {
            debug!("Invalid input.");
            (definition, game_state, String::new())
        }
    } else {
        if let Some(letter) = utils::is_any_letter_pressed() {
            (definition, game_state, format!("{}{}", input, letter))
        } else {
            (definition, game_state, input)
        }
    }
}

fn draw(definition: &WorldDefinition, game_state: &GameState, input: &String) {
    rendering::draw_scene(&definition, &game_state, &input);
    rendering::draw_ui(&definition, &game_state, &input);

    // rendering::draw_ui_debug(&game_state);
}
