#[derive(Clone)]
pub enum SpellEffectType {
    Damage,
    Healing,
}

#[derive(Clone)]
pub struct Spell {
    pub effect_type: SpellEffectType,
    pub effect_base_power: u8,
}

impl Spell {
    pub fn get_type(&self) -> &SpellEffectType {
        &self.effect_type
    }
    pub fn get_base_power(&self) -> u8 {
        self.effect_base_power
    }
}
