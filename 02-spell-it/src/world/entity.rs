use crate::utils;

type BaseHealth = u8;
type BaseDamage = u8;
type PotentialMargin = f32;
type Speed = u8;

pub enum EntityType {
    Avatar(BaseHealth, Speed),

    Goblin(BaseHealth, BaseDamage, PotentialMargin, Speed),
    Orc(BaseHealth, BaseDamage, PotentialMargin, Speed),
    Succubus(BaseHealth, BaseDamage, PotentialMargin, Speed),
}

pub struct Entity {
    identifier: String,
    name: String,
    entity_type: EntityType,
    health_max: i16,
    health: i16,
    shield: i16,
    damage: u8,
    speed: u8,
    is_alive: bool,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        match entity_type {
            EntityType::Avatar(base_health, speed) => Self {
                identifier: utils::generate_identifier("avt"),
                name: "Avatar".to_owned(),
                entity_type,
                health_max: base_health as i16,
                health: base_health as i16,
                shield: 0,
                damage: 0,
                speed,
                is_alive: true,
            },
            EntityType::Goblin(base_health, base_damage, potential, speed) => {
                let total_health = (base_health as f32 * (1f32 + potential)) as i16;

                Self {
                    identifier: utils::generate_identifier("gob"),
                    name: "Goblin".to_owned(),
                    entity_type,
                    health: total_health,
                    health_max: total_health,
                    shield: 0,
                    damage: (base_damage as f32 * (1f32 + potential)) as u8,
                    speed,
                    is_alive: true,
                }
            }
            EntityType::Orc(base_health, base_damage, potential, speed) => {
                let total_health = (base_health as f32 * (1f32 + potential)) as i16;

                Self {
                    identifier: utils::generate_identifier("orc"),
                    name: "Orc".to_owned(),
                    entity_type,
                    health: (base_health as f32 * (1f32 + potential)) as i16,
                    health_max: total_health,
                    shield: 0,
                    damage: (base_damage as f32 * (1f32 + potential)) as u8,
                    speed,
                    is_alive: true,
                }
            }
            EntityType::Succubus(base_health, base_damage, potential, speed) => {
                let total_health = (base_health as f32 * (1f32 + potential)) as i16;

                Self {
                    identifier: utils::generate_identifier("suc"),
                    name: "Succubus".to_owned(),
                    entity_type,
                    health: (base_health as f32 * (1f32 + potential)) as i16,
                    health_max: total_health,
                    shield: 0,
                    damage: (base_damage as f32 * (1f32 + potential)) as u8,
                    speed,
                    is_alive: true,
                }
            }
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_identifier(&self) -> &String {
        &self.identifier
    }

    pub fn get_entity_type(&self) -> &EntityType {
        &&self.entity_type
    }

    pub fn get_health(&self) -> &i16 {
        &self.health
    }

    pub fn get_health_max(&self) -> &i16 {
        &self.health_max
    }

    pub fn get_speed(&self) -> &u8 {
        &self.speed
    }

    pub fn heal(&mut self, amount: i16) {
        let mut final_amount = self.health + amount;

        if final_amount > self.health_max {
            final_amount = self.health_max;
        }

        self.health = final_amount;
    }

    pub fn increase_shield(&mut self, amount: i16) {
        self.shield = amount;
    }

    pub fn inflict_damage(&mut self, amount: i16) -> (i16, i16) {
        let mut mitigated_amount = amount;

        if self.shield > 0 {
            mitigated_amount -= self.shield;
            self.shield -= amount;
        }

        if self.shield < 0 {
            self.shield = 0;
        }

        if mitigated_amount < 0 {
            mitigated_amount = 0;
        }

        self.health -= mitigated_amount;

        if self.health <= 0 {
            self.is_alive = false;
        }

        (mitigated_amount, amount - mitigated_amount)
    }

    pub fn damage(&self) -> &u8 {
        &self.damage
    }

    pub fn shield(&self) -> &i16 {
        &self.shield
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
