use crate::world_definition::{Spell, SpellEffectType};

type BaseHealth = u8;
type BaseDamage = u8;
type PotentialMargin = f32;
type Speed = u8;

pub enum EnemyType {
    Goblin(BaseHealth, BaseDamage, PotentialMargin, Speed),
    Orc(BaseHealth, BaseDamage, PotentialMargin, Speed),
    Succubus(BaseHealth, BaseDamage, PotentialMargin, Speed),
}

pub struct Enemy {
    enemy_type: EnemyType,
    health: i16,
    damage: u8,
    speed: u8,
    is_alive: bool,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Goblin(base_health, base_damage, potential, speed) => Self {
                enemy_type: enemy_type,
                health: (base_health as f32 * (1f32 + potential)) as i16,
                damage: (base_damage as f32 * (1f32 + potential)) as u8,
                speed: speed,
                is_alive: true,
            },
            EnemyType::Orc(base_health, base_damage, potential, speed) => Self {
                enemy_type: enemy_type,
                health: (base_health as f32 * (1f32 + potential)) as i16,
                damage: (base_damage as f32 * (1f32 + potential)) as u8,
                speed: speed,
                is_alive: true,
            },
            EnemyType::Succubus(base_health, base_damage, potential, speed) => Self {
                enemy_type: enemy_type,
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

    pub fn get_enemy_type(&self) -> &EnemyType {
        &&self.enemy_type
    }

    pub fn get_health(&self) -> &i16 {
        &self.health
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

    pub fn get_health_max(&self) -> &i16 {
        &self.health_max
    }
    pub fn get_health(&self) -> &i16 {
        &self.health
    }
}

pub struct Stage {
    description: String,
    number: u8,
    enemies: Vec<Enemy>,
}

impl Stage {
    pub fn inflict_damage(&mut self, amount: &u8) {
        crate::debug!(
            "Inflicting damage {} on something {}",
            amount,
            self.enemies.len()
        );

        let mut inflicted = false;

        for e in self.enemies.iter_mut() {
            crate::debug!("YOLO");

            if e.is_alive() {
                if inflicted == false {
                    e.health -= *amount as i16;
                    inflicted = true;

                    if e.health <= 0 {
                        e.is_alive = false;
                        crate::debug!("BOOM");
                    }
                }
            }
        }
    }

    pub fn are_all_dead(&self) -> bool {
        self.enemies.iter().all(|e| e.is_alive == false)
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_number(&self) -> &u8 {
        &self.number
    }

    pub fn get_enemies(&self) -> &Vec<Enemy> {
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

    pub fn has_more_stages(&self) -> bool {
        self.stages.len() > 0
    }

    pub fn initialize_stages(&mut self) {
        self.stages.push(Stage {
            description: "A goblin. Shouldn't be too hard.".to_owned(),
            number: 0,
            enemies: vec![Enemy::new(EnemyType::Goblin(10, 10, 0f32, 10))],
        });

        self.stages.push(Stage {
            description: "Four more then. And a tougher one too!".to_owned(),
            number: 1,
            enemies: vec![
                Enemy::new(EnemyType::Goblin(100, 20, 0f32, 10)),
                Enemy::new(EnemyType::Goblin(10, 10, 0f32, 10)),
                Enemy::new(EnemyType::Goblin(10, 10, 0f32, 10)),
                Enemy::new(EnemyType::Goblin(10, 10, 0f32, 10)),
                Enemy::new(EnemyType::Goblin(10, 10, 0f32, 10)),
            ],
        });

        self.stages.push(Stage {
            description: "Orcs...!".to_owned(),
            number: 2,
            enemies: vec![
                Enemy::new(EnemyType::Orc(40, 25, 0f32, 10)),
                Enemy::new(EnemyType::Orc(40, 25, 0f32, 10)),
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
                        let spell_power = spell.get_base_power();

                        stage.inflict_damage(&spell_power);

                        if stage.are_all_dead() {
                            crate::debug!("All Dead");
                            GameStateUpdateResult::NextStage
                        } else {
                            crate::debug!("Not all dead");
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
