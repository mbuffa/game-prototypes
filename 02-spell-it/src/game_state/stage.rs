use crate::world::entity::Entity;

pub struct Stage {
  pub description: String,
  pub number: u8,
  pub enemies: Vec<Entity>,
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

  pub fn inflict_damage_to_all(&mut self, spell_power: i16) {
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
