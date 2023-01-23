use crate::game_state::GameState;
use crate::world_definition::WorldDefinition;
use crate::systems;
use crate::utils;

use macroquad::prelude::*;

pub struct GameMaster {
    definition: WorldDefinition,
    game_state: GameState,
    player_input: String
}

impl GameMaster {
    pub fn new() -> Self {
        Self {
            definition: WorldDefinition::new(),
            game_state: GameState::new(),
            player_input: String::new()
        }
    }
    
    pub fn start_session(&mut self) {
        self.definition.initialize_spell_types();
        self.game_state.initialize();
    }
    
    pub fn update(&mut self, since_last_frame: &f32) -> (&WorldDefinition, &GameState, &String) {
        systems::maybe_go_to_next_stage(&mut self.game_state);
        systems::maybe_mark_game_as_over_or_won(&mut self.game_state);
        
        if self.game_state.is_won() {
            crate::debug!("You won!");
        }
        
        if is_key_pressed(KeyCode::Enter) {
            let sanitized = systems::sanitize_input(&self.player_input);
            
            if systems::validate_input(&self.definition, &sanitized) {
                match systems::execute_input(&self.definition, &mut self.game_state, &sanitized) {
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

        (&self.definition, &self.game_state, &self.player_input)
    }
    
    pub fn get_definition(&self) -> &WorldDefinition { &self.definition }
    pub fn get_game_state(&self) -> &GameState { &self.game_state }
    pub fn get_player_input(&self) -> &String { &self.player_input }
}