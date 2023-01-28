use macroquad::prelude::*;

use crate::world::entity::Entity;

const TIME_FOR_SEQUENCE_TRANSITION: f32 = 0.5f32;

pub enum CombatState {
  Idle,
  PlayerTurn,
  EnemyTurn,
}

pub struct Sequence {
  order: Vec<(String, u8)>,
  state: CombatState,
  current: usize,
  in_transition: bool,
  transition_time: f32,
}

impl Sequence {
  pub fn from(player: &Entity, enemies: &Vec<Entity>) -> Self {
      let mut order = Vec::new();

      order.push((player.get_identifier().clone(), player.get_speed().clone()));
      enemies
          .iter()
          .for_each(|e| order.push((e.get_identifier().clone(), e.get_speed().clone())));

      order.sort_by(|a, b| b.1.cmp(&a.1));

      let state = match order.first() {
          None => panic!("Something weird happened in Sequence draft."),
          Some((identifier, _)) => {
              if identifier == player.get_identifier() {
                  CombatState::PlayerTurn
              } else {
                  CombatState::EnemyTurn
              }
          }
      };

      Self {
          order,
          state,
          current: 0,
          in_transition: false,
          transition_time: 0f32,
      }
  }

  pub fn get_order(&self) -> &Vec<(String, u8)> {
      &self.order
  }

  pub fn get_state(&self) -> &CombatState {
      &self.state
  }

  pub fn set_state(&mut self, state: CombatState) {
      self.state = state;
  }

  pub fn current(&self) -> usize {
      self.current
  }

  pub fn in_transition(&self) -> bool {
      self.in_transition
  }

  pub fn next(&mut self) {
      if self.in_transition == false {
          self.in_transition = true;
      }
  }

  fn go_next(&mut self) {
      self.current += 1;
      self.in_transition = false;
      self.transition_time = 0f32;
  }

  pub fn reset(&mut self) {
      self.current = 0;
  }

  pub fn tick(&mut self) {
      if self.in_transition {
          self.transition_time += get_frame_time();
      }

      if self.in_transition && self.transition_time >= TIME_FOR_SEQUENCE_TRANSITION {
          self.in_transition = false;
          self.transition_time = 0f32;
          self.go_next();
      }
  }
}
