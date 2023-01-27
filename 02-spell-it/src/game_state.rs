use crate::world::entity::{Entity, EntityType};
use crate::world::spell::{Spell, SpellEffectType};

pub struct Stage {
    description: String,
    number: u8,
    enemies: Vec<Entity>,
}

impl Stage {
    pub fn inflict_damage(&mut self, amount: i16) {
        crate::debug!(
            "Inflicting damage {} on something {}",
            amount,
            self.enemies.len()
        );

        let mut inflicted = false;

        for e in self.enemies.iter_mut() {
            crate::debug!("YOLO");

            if e.is_alive() && inflicted == false {
                e.inflict_damage(amount);
                inflicted = true;
            }
        }
    }

    fn inflict_damage_to_all(&mut self, spell_power: i16) {
        self.enemies.iter_mut().for_each(|e| {
            if e.is_alive() {
                e.inflict_damage(spell_power);
            }
        });
    }

    pub fn are_all_dead(&self) -> bool {
        self.enemies.iter().all(|e| e.is_alive() == false)
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_owned();
    }

    pub fn get_number(&self) -> &u8 {
        &self.number
    }

    pub fn get_enemies(&self) -> &Vec<Entity> {
        &self.enemies
    }
}

#[derive(Debug, PartialEq)]
pub enum GameStateUpdateResult {
    Error,
    NextTurn,
    NextStage,
    GameOver,
    GameWon,
}

pub struct Scene {
    stages: Vec<Stage>,
    player: Entity,
}

impl Scene {
    pub fn get_player(&self) -> &Entity {
        &self.player
    }
    pub fn get_player_mut(&mut self) -> &mut Entity {
        &mut self.player
    }

    pub fn get_current_stage(&self) -> Option<&Stage> {
        self.stages.last()
    }

    pub fn get_current_stage_mut(&mut self) -> Option<&mut Stage> {
        self.stages.last_mut()
    }

    pub fn player_and_stage_mut(&mut self) -> (&mut Entity, Option<&mut Stage>) {
        (&mut self.player, self.stages.last_mut())
    }

    pub fn go_to_next_stage(&mut self) {
        self.stages.pop();
    }

    pub fn has_more_stages(&self) -> bool {
        self.stages.len() > 0
    }

    pub fn initialize_stages(&mut self) {
        self.stages.push(Stage {
            description: "A goblin. Shouldn't be too hard...".to_owned(),
            number: 0,
            enemies: vec![Entity::new(EntityType::Goblin(10, 10, 0f32, 10))],
        });

        self.stages.push(Stage {
            description: "Four little ones. But mama is here!".to_owned(),
            number: 1,
            enemies: vec![
                Entity::new(EntityType::Goblin(100, 20, 0f32, 10)),
                Entity::new(EntityType::Goblin(10, 5, 0f32, 10)),
                Entity::new(EntityType::Goblin(10, 5, 0f32, 10)),
                Entity::new(EntityType::Goblin(10, 5, 0f32, 10)),
                Entity::new(EntityType::Goblin(10, 5, 0f32, 10)),
            ],
        });

        self.stages.push(Stage {
            description: "Orcs...!".to_owned(),
            number: 2,
            enemies: vec![
                Entity::new(EntityType::Orc(40, 25, 0f32, 10)),
                Entity::new(EntityType::Orc(40, 25, 0f32, 10)),
            ],
        });

        self.stages.reverse();
    }

    pub fn update_with(&mut self, spell: Option<&Spell>) -> GameStateUpdateResult {
        let current_stage = self.stages.last_mut();

        match current_stage {
            None => GameStateUpdateResult::Error,
            Some(stage) => match spell {
                None => {
                    panic!("No stage found!")
                }
                Some(spell) => match spell.get_type().clone() {
                    SpellEffectType::Damage => {
                        let spell_power = spell.get_base_power() as i16;

                        stage.inflict_damage(spell_power);

                        if stage.are_all_dead() {
                            crate::debug!("All Dead");
                            GameStateUpdateResult::NextStage
                        } else {
                            crate::debug!("Not all dead");
                            GameStateUpdateResult::NextTurn
                        }
                    }
                    SpellEffectType::Healing => {
                        let spell_power = spell.get_base_power() as i16;

                        self.player.heal(spell_power);

                        GameStateUpdateResult::NextTurn
                    }
                    SpellEffectType::Shield => {
                        let spell_power = spell.get_base_power() as i16;

                        self.player.increase_shield(spell_power);

                        GameStateUpdateResult::NextTurn
                    }
                    SpellEffectType::MultiDamage => {
                        let spell_power = spell.get_base_power() as i16;

                        stage.inflict_damage_to_all(spell_power);

                        if stage.are_all_dead() {
                            crate::debug!("All Dead");
                            GameStateUpdateResult::NextStage
                        } else {
                            crate::debug!("Not all dead");
                            GameStateUpdateResult::NextTurn
                        }
                    }
                },
            },
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.player.is_alive() == false
    }

    pub fn is_game_won(&self) -> bool {
        if self.has_more_stages() {
            return false;
        }

        if let Some(stage) = self.get_current_stage() {
            stage.are_all_dead()
        } else {
            true
        }
    }
}

pub struct GameState {
    scene: Scene,
    is_over: bool,
    is_won: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            scene: Scene {
                player: Entity::new(EntityType::Avatar(100, 70)),
                stages: Vec::new(),
            },
            is_over: false,
            is_won: false,
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
}
