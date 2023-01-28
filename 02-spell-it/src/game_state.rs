pub mod stage;
pub mod scene;

use scene::Scene;

use crate::world::entity::{Entity, EntityType};

#[derive(Debug, PartialEq)]
pub enum GameStateUpdateResult {
    Error,
    NextTurn,
    NextStage,
    GameOver,
    GameWon,
}

pub struct GameState {
    scene: Scene,
    is_over: bool,
    is_won: bool,
    is_player_turn: bool,
    in_transition: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(Entity::new(EntityType::Avatar(100, 70))),
            is_over: false,
            is_won: false,
            is_player_turn: false,
            in_transition: false,
        }
    }

    pub fn initialize(&mut self) {
        self.scene.initialize_stages();
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn is_won(&self) -> bool {
        self.is_won
    }

    pub fn set_is_over(&mut self, over: bool) {
        self.is_over = over;
    }

    pub fn set_is_won(&mut self, won: bool) {
        self.is_won = won;
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scene
    }

    pub fn get_scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn is_player_turn(&self) -> bool {
        self.is_player_turn
    }

    pub fn set_is_player_turn(&mut self, is_player_turn: bool) {
        self.is_player_turn = is_player_turn;
    }

    pub fn set_in_transition(&mut self, in_transition: bool) {
        self.in_transition = in_transition;
    }
}
