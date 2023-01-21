use std::collections::HashMap;

#[derive(Clone)]
pub enum SpellEffectType {
    Damage,
    Healing,
}

#[derive(Clone)]
pub struct Spell {
    effect_type: SpellEffectType,
    effect_base_power: u8,
}

impl Spell {
    pub fn get_type(&self) -> &SpellEffectType {
        &self.effect_type
    }
    pub fn get_base_power(&self) -> u8 {
        self.effect_base_power
    }
}

pub struct WorldDefinition {
    spell_types: HashMap<String, Spell>,
}

impl WorldDefinition {
    pub fn new() -> Self {
        Self {
            spell_types: HashMap::new(),
        }
    }

    pub fn get_spell_types(&self) -> &HashMap<String, Spell> {
        &self.spell_types
    }

    pub fn initialize_spell_types(&mut self) {
        self.spell_types.insert(
            "Wololo".to_owned(),
            Spell {
                effect_type: SpellEffectType::Healing,
                effect_base_power: 25,
            },
        );

        self.spell_types.insert(
            "Awo you you".to_owned(),
            Spell {
                effect_type: SpellEffectType::Damage,
                effect_base_power: 12,
            },
        );
    }
}
