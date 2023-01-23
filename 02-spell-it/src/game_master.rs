use crate::game_state::GameState;
use crate::systems;
use crate::utils;
use crate::world::entity::{Entity, EntityType};
use crate::world::World;

use macroquad::prelude::*;

enum State {
    Uninitialized,
    Initialized,
}

// struct Sequence {
//     order: Vec<String>
// }

// impl Sequence {
//     pub fn from(player: &Player, enemies: &Vec<Enemy>) -> Self {
//         let discriminator = 1;
//         let mut order = Vec::new();
//         order.push(player);

//         enemies.iter().for_each(|e| order.push(e));

//         Self {
//             order: Vec::new()
//         }
//     }

//     pub fn get_order(&self) -> &Vec<String> { &self.order }
// }

pub struct GameMaster {
    world: World,
    game_state: GameState,
    player_input: String,
    state: State,
    current_turn: u16,
    // sequence: Option<Sequence>
}

impl GameMaster {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            game_state: GameState::new(),
            player_input: String::new(),
            state: State::Uninitialized,
            current_turn: 0,
            // sequence: None
        }
    }

    pub fn start_session(&mut self) {
        self.world.initialize_spell_types();
        self.game_state.initialize();
        self.state = State::Initialized;
    }

    pub fn update(&mut self, _since_last_frame: &f32) -> (&World, &GameState, &String) {
        match self.state {
            State::Uninitialized => panic!("GameMaster is uninitialized!"),
            _ => {}
        }
        systems::maybe_go_to_next_stage(&mut self.game_state);
        systems::maybe_mark_game_as_over_or_won(&mut self.game_state);

        if self.game_state.is_won() {
            crate::debug!("You won!");
        }

        if is_key_pressed(KeyCode::Enter) {
            let sanitized = systems::sanitize_input(&self.player_input);

            if systems::validate_input(&self.world, &sanitized) {
                match systems::execute_input(&self.world, &mut self.game_state, &sanitized) {
                    _ => crate::debug!("Updated Scene."),
                }

                self.player_input = String::new();
            } else {
                crate::debug!("Invalid input.");
                self.player_input = String::new();
            }
        } else {
            if let Some(letter) = utils::is_any_letter_pressed() {
                self.player_input = format!("{}{}", &self.player_input, letter);
            }
        }

        (&self.world, &self.game_state, &self.player_input)
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }
    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }
    pub fn get_player_input(&self) -> &String {
        &self.player_input
    }
}
