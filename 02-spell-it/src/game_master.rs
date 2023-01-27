use crate::game_state::GameState;
use crate::game_state::Scene;
use crate::systems;
use crate::utils;
use crate::world::entity::Entity;
use crate::world::World;

use macroquad::prelude::*;

const TIME_FOR_SEQUENCE_TRANSITION: f32 = 2f32;
const TIME_FOR_STAGE_TRANSITION: f32 = 3f32;

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
    state: CombatState,
    current: usize,
    in_transition: bool,
    transition_time: f32,
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
            state,
            current: 0,
            in_transition: false,
            transition_time: 0f32,
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

    pub fn in_transition(&self) -> bool {
        self.in_transition
    }

    pub fn next(&mut self) {
        if self.in_transition == false {
            println!("Started sequence transition");
            self.in_transition = true;
        }
    }

    fn go_next(&mut self) {
        println!("Going to next");
        self.current += 1;
        self.in_transition = false;
        self.transition_time = 0f32;
    }

    pub fn reset(&mut self) {
        println!("Reset");
        self.current = 0;
    }

    pub fn tick(&mut self) {
        if self.in_transition {
            self.transition_time += get_frame_time();
        }

        if self.in_transition && self.transition_time >= TIME_FOR_SEQUENCE_TRANSITION {
            println!("Reached elapsed time target");
            self.in_transition = false;
            self.transition_time = 0f32;
            self.go_next();
        }
    }
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

        if self.in_transition {
            self.transition_time += get_frame_time();
        }

        if systems::can_go_to_next_stage(&mut self.game_state) {
            // Starting transition
            if self.in_transition == false {
                set_description(&mut self.game_state, "Let's go forward!");
                self.in_transition = true;
            }

            // Achieving transition
            if self.in_transition && self.transition_time >= TIME_FOR_STAGE_TRANSITION {
                self.in_transition = false;
                self.transition_time = 0f32;
                self.game_state.get_scene_mut().go_to_next_stage();
                self.calculate_sequence();
            }
        }

        systems::maybe_mark_game_as_over_or_won(&mut self.game_state);

        if self.game_state.is_won() {
            // crate::debug!("You won!");
            // FIXME: Play victory sound.
        } else if self.game_state.is_over() {
            // crate::debug!("You lost!");
            // FIXME: Play defeat sound.
        } else {
            self.update_combat_state();

            match &self.sequence {
                Some(sequence) => {
                    if sequence.in_transition() == false {
                        self.update_combat_system();
                    }
                }
                None => {}
            }
        }

        (&self.world, &self.game_state, &self.player_input)
    }

    fn update_combat_state(&mut self) {
        // crate::debug!("Entering update_combat_state");
        let player_identifier = self.player_identifier.as_ref().expect("Yolo");

        match &mut self.sequence {
            None => panic!("Holy cwap"),
            Some(sequence) => {
                sequence.tick();

                match sequence.get_order().get(sequence.current()) {
                    None => {
                        // crate::debug!("Sequence end reached");
                        // crate::debug!("{:?}", sequence.order);
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
    }

    fn update_combat_system(&mut self) {
        // crate::debug!("Entering update_combat_system");

        match &mut self.sequence {
            Some(sequence) => {
                match sequence.get_state() {
                    CombatState::Idle => {
                        crate::debug!("Combat Idle");
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

fn set_description(game_state: &mut GameState, arg: &str) {
    match game_state.get_scene_mut().get_current_stage_mut() {
        None => panic!("Woopsie"),
        Some(stage) => {
            stage.set_description(arg);
        }
    }
}
