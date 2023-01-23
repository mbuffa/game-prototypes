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
    entity_type: EntityType,
    health_max: i16,
    health: i16,
    damage: u8,
    speed: u8,
    is_alive: bool,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        match entity_type {
            EntityType::Avatar(base_health, speed) => Self {
                identifier: utils::generate_identifier("avt"),
                entity_type,
                health_max: base_health as i16,
                health: base_health as i16,
                damage: 0,
                speed,
                is_alive: true,
            },
            EntityType::Goblin(base_health, base_damage, potential, speed) => {
                let total_health = (base_health as f32 * (1f32 + potential)) as i16;

                Self {
                    identifier: utils::generate_identifier("gob"),
                    entity_type,
                    health: total_health,
                    health_max: total_health,
                    damage: (base_damage as f32 * (1f32 + potential)) as u8,
                    speed,
                    is_alive: true,
                }
            }
            EntityType::Orc(base_health, base_damage, potential, speed) => {
                let total_health = (base_health as f32 * (1f32 + potential)) as i16;

                Self {
                    identifier: utils::generate_identifier("orc"),
                    entity_type,
                    health: (base_health as f32 * (1f32 + potential)) as i16,
                    health_max: total_health,
                    damage: (base_damage as f32 * (1f32 + potential)) as u8,
                    speed,
                    is_alive: true,
                }
            }
            EntityType::Succubus(base_health, base_damage, potential, speed) => {
                let total_health = (base_health as f32 * (1f32 + potential)) as i16;

                Self {
                    identifier: utils::generate_identifier("suc"),
                    entity_type,
                    health: (base_health as f32 * (1f32 + potential)) as i16,
                    health_max: total_health,
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
        let final_amount = self.health_max - self.health + amount;
        self.health += final_amount;
    }

    pub fn damage(&mut self, amount: i16) {
        self.health -= amount;

        if self.health <= 0 {
            self.is_alive = false;
            crate::debug!("BOOM");
        }
    }
}
