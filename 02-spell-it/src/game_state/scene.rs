use crate::world::entity::{Entity, EntityType};
use crate::world::spell::{Spell, SpellEffectType};

use super::GameStateUpdateResult;
use super::stage::Stage;

pub struct Scene {
    stages: Vec<Stage>,
    player: Entity,
}

impl Scene {
    pub fn new(player: Entity) -> Self {
        Self {
            player,
            stages: Vec::new()
        }
    }
    
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
            enemies: vec![Entity::new(EntityType::Goblin(10, 8, 0f32, 10))],
        });
        
        self.stages.push(Stage {
            description: "Four little ones. But mama is here!".to_owned(),
            number: 1,
            enemies: vec![
            Entity::new(EntityType::Goblin(70, 12, 0f32, 10)),
            Entity::new(EntityType::Goblin(10, 8, 0f32, 10)),
            Entity::new(EntityType::Goblin(10, 8, 0f32, 10)),
            Entity::new(EntityType::Goblin(10, 8, 0f32, 10)),
            Entity::new(EntityType::Goblin(10, 8, 0f32, 10)),
            ],
        });
        
        self.stages.push(Stage {
            description: "Orcs...!".to_owned(),
            number: 2,
            enemies: vec![
            Entity::new(EntityType::Orc(40, 14, 0f32, 10)),
            Entity::new(EntityType::Orc(40, 14, 0f32, 10)),
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
                            GameStateUpdateResult::NextStage
                        } else {
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
                            GameStateUpdateResult::NextStage
                        } else {
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