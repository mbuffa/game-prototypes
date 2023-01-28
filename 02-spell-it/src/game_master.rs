use crate::game_state::GameState;
use crate::game_state::scene::Scene;
use crate::systems;
use crate::utils;
use crate::world::World;
use crate::combat::sequence::{Sequence, CombatState};

use macroquad::prelude::*;

const TIME_FOR_STAGE_TRANSITION: f32 = 2f32;

enum State {
    Uninitialized,
    Initialized,
    SequenceDrafted,
}

pub struct GameMaster {
    world: World,
    game_state: GameState,
    player_input: String,
    player_identifier: Option<String>,
    state: State,
    in_transition: bool,
    transition_time: f32,
    // FIXME: Move those two to GameState to expose them to draw functions.
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
            in_transition: false,
            transition_time: 0f32,
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

        if self.in_transition {
            self.transition_time += get_frame_time();
        }

        systems::maybe_mark_game_as_over_or_won(&mut self.game_state);

        if systems::can_go_to_next_stage(&mut self.game_state) {
            // Starting transition
            if self.in_transition == false {
                set_description(&mut self.game_state, "Let's go forward!");
                self.in_transition = true;
                self.game_state.set_in_transition(true);
            }

            // Achieving transition
            if self.in_transition && self.transition_time >= TIME_FOR_STAGE_TRANSITION {
                self.in_transition = false;
                self.game_state.set_in_transition(false);
                self.transition_time = 0f32;
                self.game_state.get_scene_mut().go_to_next_stage();

                if let Some(_) = self.game_state.get_scene().get_current_stage() {
                    self.calculate_sequence();
                }
            }
        }

        if self.game_state.is_won() {
            // FIXME: Play victory sound.
        } else if self.game_state.is_over() {
            // FIXME: Play defeat sound.
        } else {
            self.update_combat_state();

            match &self.sequence {
                Some(sequence) => {
                    if sequence.in_transition() {
                        self.game_state.set_in_transition(true);
                        self.game_state.set_is_player_turn(false);
                    } else {
                        self.update_combat_system();
                        self.game_state.set_in_transition(false);
                    }
                }
                None => {}
            }
        }

        (&self.world, &self.game_state, &self.player_input)
    }

    fn update_combat_state(&mut self) {
        let player_identifier = self.player_identifier.as_ref().expect("Yolo");

        if let Some(sequence) = &mut self.sequence {
            sequence.tick();

            match sequence.get_order().get(sequence.current()) {
                None => {
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
            }
        }
    }

    fn update_combat_system(&mut self) {
        match &mut self.sequence {
            Some(sequence) => {
                match sequence.get_state() {
                    CombatState::Idle => {
                        self.game_state.set_is_player_turn(false);
                    }
                    CombatState::PlayerTurn => {
                        self.game_state.set_is_player_turn(true);

                        if is_key_pressed(KeyCode::Enter) {
                            let sanitized = systems::sanitize_input(&self.player_input);

                            self.game_state.set_in_transition(true);

                            if systems::validate_input(&self.world, &sanitized) {
                                systems::execute_input(
                                    &self.world,
                                    &mut self.game_state,
                                    &sanitized,
                                );

                                self.player_input = String::new();
                                sequence.next();
                            } else {
                                set_description(&mut self.game_state, "Fizzle!");
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
                        self.game_state.set_is_player_turn(false);

                        let scene: &mut Scene = self.game_state.get_scene_mut();
                        let (player, current_stage) = scene.player_and_stage_mut();

                        match current_stage {
                            None => panic!("Woops"),
                            Some(stage) => {
                                match sequence.get_order().get(sequence.current()) {
                                    None => panic!("Woop Woop"),
                                    Some((identifier, _speed)) => {
                                        match stage.get_enemies().iter().find(|e| {
                                            e.is_alive() && e.get_identifier() == identifier
                                        }) {
                                            None => {
                                                // The entity that was supposed to act is dead. We go to the next sequence.
                                            }
                                            Some(e) => {
                                                let enemy_name = e.name();
                                                let amount = *e.damage() as i16;

                                                let (inflicted, absorbed) =
                                                    player.inflict_damage(amount);
                                                let player_name = player.name();

                                                let description = format!(
                                                    "{} inflicted {} to {} ({} absorbed)",
                                                    enemy_name, &inflicted, player_name, &absorbed
                                                );

                                                set_description(
                                                    &mut self.game_state,
                                                    description.as_str(),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        sequence.next();
                    }
                }
            }
            None => {
                panic!("No sequence.")
            }
        }
    }
}

fn set_description(game_state: &mut GameState, arg: &str) {
    match game_state.get_scene_mut().get_current_stage_mut() {
        None => panic!("Woopsie"),
        Some(stage) => {
            stage.set_description(arg);
        }
    }
}
