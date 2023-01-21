use crate::world_definition::{Spell, SpellEffectType};

type BaseHealth = u8;
type BaseDamage = u8;
type PotentialMargin = f32;
type Speed = u8;

enum EnemyType {
    Goblin(BaseHealth, BaseDamage, PotentialMargin, Speed),
    Orc(BaseHealth, BaseDamage, PotentialMargin, Speed),
    Succubus(BaseHealth, BaseDamage, PotentialMargin, Speed),
}

struct Enemy {
    health: i16,
    damage: u8,
    speed: u8,
    is_alive: bool,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Goblin(base_health, base_damage, potential, speed) => Self {
                health: (base_health as f32 * (1f32 + potential)) as i16,
                damage: (base_damage as f32 * (1f32 + potential)) as u8,
                speed: speed,
                is_alive: true,
            },
            EnemyType::Orc(base_health, base_damage, potential, speed) => Self {
                health: (base_health as f32 * (1f32 + potential)) as i16,
                damage: (base_damage as f32 * (1f32 + potential)) as u8,
                speed: speed,
                is_alive: true,
            },
            EnemyType::Succubus(base_health, base_damage, potential, speed) => Self {
                health: (base_health as f32 * (1f32 + potential)) as i16,
                damage: (base_damage as f32 * (1f32 + potential)) as u8,
                speed: speed,
                is_alive: true,
            },
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

pub struct Player {
    health_max: i16,
    health: i16,
    speed: u8,
    is_alive: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health_max: 100,
            health: 100,
            speed: 10,
            is_alive: true,
        }
    }

    pub fn heal(&mut self, amount: u8) {
        let final_amount = self.health_max - self.health + amount as i16;
        self.health += final_amount;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

pub struct Stage {
    number: u16,
    enemies: Vec<Enemy>,
}

impl Stage {
    pub fn inflict_damage(&mut self, amount: &u8) {
        println!(
            "Inflicting damage {} on something {}",
            amount,
            self.enemies.len()
        );

        let mut inflicted = false;

        for e in self.enemies.iter_mut() {
            println!("YOLO");

            if e.is_alive() {
                if inflicted == false {
                    e.health -= *amount as i16;
                    inflicted = true;

                    if e.health <= 0 {
                        e.is_alive = false;
                        println!("BOOM");
                    }
                }
            }
        }
    }

    pub fn are_all_dead(&self) -> bool {
        self.enemies.iter().all(|e| e.is_alive == false)
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
    player: Player,
}

impl Scene {
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn get_current_stage(&self) -> Option<&Stage> {
        self.stages.last()
    }
    pub fn get_current_stage_mut(&mut self) -> Option<&mut Stage> {
        self.stages.last_mut()
    }

    pub fn go_to_next_stage(&mut self) {
        self.stages.pop();
    }

    pub fn initialize_stages(&mut self) {
        self.stages.push(Stage {
            number: 0,
            enemies: vec![Enemy::new(EnemyType::Goblin(10, 10, 0f32, 10))],
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
                        let spell_power = spell.get_base_power();

                        stage.inflict_damage(&spell_power);

                        if stage.are_all_dead() {
                            println!("All Dead");
                            GameStateUpdateResult::NextStage
                        } else {
                            println!("Not all dead");
                            GameStateUpdateResult::NextTurn
                        }
                    }
                    SpellEffectType::Healing => {
                        let spell_power = spell.get_base_power();

                        self.player.heal(spell_power);

                        GameStateUpdateResult::NextTurn
                    }
                },
            },
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.player.is_alive() == false
    }

    pub fn is_game_won(&self) -> bool {
        if let Some(stage) = self.get_current_stage() {
            stage.are_all_dead()
        } else {
            false
        }
    }
}

pub struct GameState {
    current_turn: u16,
    scene: Scene,
    is_over: bool,
    is_won: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_turn: 0,
            scene: Scene {
                player: Player {
                    health_max: 100,
                    health: 100,
                    speed: 10,
                    is_alive: true,
                },
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
