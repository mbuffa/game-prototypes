use std::ops::Index;

use crate::game_state::GameState;
use crate::systems;
use crate::utils;
use crate::world::entity::{Entity, EntityType};
use crate::world::World;

use macroquad::prelude::*;

enum CombatState {
    Idle,
    PlayerTurn,
    EnemyTurn,
}

enum State {
    Uninitialized,
    Initialized,
    SequenceDrafted,
}

struct Sequence {
    order: Vec<(String, u8)>,
    // iter_fct: fn() -> dyn Iterator<Item = &'a (String, u8)>,
    // iter: Box<dyn Iterator<Item = &'a (String, u8)>>
    state: CombatState,
    current: usize,
}

impl Sequence {
    pub fn from(player: &Entity, enemies: &Vec<Entity>) -> Self {
        let mut order = Vec::new();

        order.push((player.get_identifier().clone(), player.get_speed().clone()));
        enemies
            .iter()
            .for_each(|e| order.push((e.get_identifier().clone(), e.get_speed().clone())));

        order.sort_by(|a, b| b.1.cmp(&a.1));

        let state = match order.first() {
            None => panic!("Something weird happened in Sequence draft."),
            Some((identifier, _)) => {
                if identifier == player.get_identifier() {
                    crate::debug!("Player Turn");
                    CombatState::PlayerTurn
                } else {
                    crate::debug!("Enemy Turn");
                    CombatState::EnemyTurn
                }
            }
        };

        Self {
            order,
            // iter: Box::new(order.iter())
            state,
            current: 0,
        }
    }

    pub fn get_order(&self) -> &Vec<(String, u8)> {
        &self.order
    }

    fn get_state(&self) -> &CombatState {
        &self.state
    }

    fn set_state(&mut self, state: CombatState) {
        self.state = state;
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn next(&mut self) {
        self.current += 1;
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}

pub struct GameMaster {
    world: World,
    game_state: GameState,
    player_input: String,
    player_identifier: Option<String>,
    state: State,
    current_turn: u16,
    sequence: Option<Sequence>,
}

impl GameMaster {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            game_state: GameState::new(),
            player_input: String::new(),
            player_identifier: None,
            state: State::Uninitialized,
            current_turn: 0,
            sequence: None,
        }
    }

    pub fn start_session(&mut self) {
        self.world.initialize_spell_types();
        self.game_state.initialize();
        self.state = State::Initialized;
        self.player_identifier = Some(
            self.game_state
                .get_scene()
                .get_player()
                .get_identifier()
                .clone(),
        );
        self.calculate_sequence();
    }

    fn calculate_sequence(&mut self) {
        crate::debug!("Enterijg calculate_sequence");
        let scene = self.game_state.get_scene();

        match scene.get_current_stage() {
            Some(stage) => {
                let sequence = Sequence::from(scene.get_player(), stage.get_enemies());
                self.sequence = Some(sequence);
                self.state = State::SequenceDrafted;
            }
            None => panic!("No stage found."),
        }
    }

    pub fn update(&mut self, _since_last_frame: &f32) -> (&World, &GameState, &String) {
        match self.state {
            State::Uninitialized => panic!("GameMaster is uninitialized!"),
            _ => {}
        }

        if systems::maybe_go_to_next_stage(&mut self.game_state) {
            self.calculate_sequence();
        }
        systems::maybe_mark_game_as_over_or_won(&mut self.game_state);

        if self.game_state.is_won() {
            crate::debug!("You won!");
        }

        self.update_combat_state();
        self.update_combat_system();

        (&self.world, &self.game_state, &self.player_input)
    }

    fn update_combat_state(&mut self) {
        crate::debug!("Entering update_combat_state");
        let player_identifier = self.player_identifier.as_ref().expect("Yolo");

        match &mut self.sequence {
            None => panic!("Holy cwap"),
            Some(sequence) => match sequence.get_order().get(sequence.current()) {
                None => {
                    crate::debug!("Sequence end reached");
                    crate::debug!("{:?}", sequence.order);
                    sequence.reset();

                    match sequence.get_order().get(sequence.current()) {
                        None => panic!("Hole sheet"),
                        Some((identifier, _speed)) => {
                            if identifier == player_identifier {
                                sequence.set_state(CombatState::PlayerTurn);
                            } else {
                                sequence.set_state(CombatState::EnemyTurn);
                            }
                        }
                    }
                }
                Some((identifier, _speed)) => {
                    if identifier == player_identifier {
                        sequence.set_state(CombatState::PlayerTurn);
                    } else {
                        sequence.set_state(CombatState::EnemyTurn);
                    }
                }
            },
        }
    }

    fn update_combat_system(&mut self) {
        crate::debug!("Entering update_combat_system");

        match &mut self.sequence {
            Some(sequence) => {
                match sequence.get_state() {
                    CombatState::Idle => {
                        crate::debug!("Combat Idle")
                    }
                    CombatState::PlayerTurn => {
                        if is_key_pressed(KeyCode::Enter) {
                            let sanitized = systems::sanitize_input(&self.player_input);

                            if systems::validate_input(&self.world, &sanitized) {
                                match systems::execute_input(
                                    &self.world,
                                    &mut self.game_state,
                                    &sanitized,
                                ) {
                                    _ => crate::debug!("Updated Scene."),
                                }

                                self.player_input = String::new();
                                sequence.next();
                            } else {
                                crate::debug!("Invalid input.");
                                self.player_input = String::new();
                                sequence.next();
                            }
                        } else {
                            if let Some(letter) = utils::is_any_letter_pressed() {
                                self.player_input = format!("{}{}", &self.player_input, letter);
                            }
                        }
                    }
                    CombatState::EnemyTurn => {
                        // FIXME: Implement AI.
                        crate::debug!("FIXME: Enemy passes his turn.");
                        sequence.next();
                    }
                }
            }
            None => {
                panic!("No sequence.")
            }
        }

        // FIXME: Pass turn.
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
