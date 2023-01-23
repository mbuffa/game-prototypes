pub mod entity;
pub mod spell;

use std::collections::HashMap;

use spell::{Spell, SpellEffectType};

pub struct World {
    spell_types: HashMap<String, Spell>,
}

impl World {
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
            "wololo".to_owned(),
            Spell {
                effect_type: SpellEffectType::Healing,
                effect_base_power: 25,
            },
        );

        self.spell_types.insert(
            "awo you you".to_owned(),
            Spell {
                effect_type: SpellEffectType::Damage,
                effect_base_power: 12,
            },
        );
    }
}
