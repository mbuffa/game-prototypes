use super::fire_mode::FireMode;

// Rate of fire, in seconds.
const NORMAL_RATE_OF_FIRE: f32 = 1f32;

// Constants (in seconds) related to Burst Mode.
const BURST_RATE_OF_FIRE: f32 = 0.2f32;
const BURST_DURATION: f32 = 1.0f32;
const BURST_COOLDOWN: f32 = 6f32;

pub struct RateTimer {
  fire_mode: FireMode,
  time_since_last_shot: f32,
  time_since_burst_start: f32,
  time_since_burst_cooldown: f32,
  is_bursting: bool
}

impl RateTimer {
  pub fn new(fire_mode: FireMode) -> Self {
    Self {
      fire_mode,
      time_since_last_shot: 0f32,
      time_since_burst_start: 0f32,
      time_since_burst_cooldown: 0f32,
      is_bursting: false
    }
  }

  pub fn increment(&mut self, dt: f32) {
    match self.fire_mode {
      FireMode::Normal => {
        self.time_since_last_shot += dt;
      },
      FireMode::Burst => {
        if self.is_bursting {
          self.time_since_last_shot += dt;
          self.time_since_burst_start += dt;
        } else {
          if self.time_since_burst_cooldown >= BURST_COOLDOWN {
            self.time_since_last_shot += dt;
            self.time_since_burst_start = 0f32;
          } else {
            self.time_since_burst_cooldown += dt;
          }
        }
      }
    }
  }

  pub fn reset(&mut self) {
    match self.fire_mode {
      FireMode::Normal => {
        self.time_since_last_shot = 0f32;
      },
      FireMode::Burst => {
        self.time_since_last_shot = 0f32;
      }
    }
  }

  pub fn can_shoot(&mut self) -> bool {
    match self.fire_mode {
      FireMode::Normal => {
        self.time_since_last_shot >= NORMAL_RATE_OF_FIRE
      },
      FireMode::Burst => {
        let can_shoot = self.time_since_last_shot >= BURST_RATE_OF_FIRE &&
                              self.time_since_burst_start < BURST_DURATION &&
                              self.time_since_burst_cooldown >= BURST_COOLDOWN;

        if !self.is_bursting && can_shoot {
          self.is_bursting = true;
        }

        can_shoot
      }
    }
  }

  pub fn update(&mut self, dt: f32) {
    match self.fire_mode {
      FireMode::Normal => {

      },
      FireMode::Burst => {
        // debug!(
        //   "{}\t{}\t{}\t{}",
        //   self.is_bursting,
        //   self.time_since_last_shot.round(),
        //   self.time_since_burst_start.round(), 
        //   self.time_since_burst_cooldown.round()
        // );

        if self.is_bursting && self.time_since_burst_start >= BURST_DURATION {
          self.is_bursting = false;
          self.time_since_burst_start = 0f32;
          self.time_since_burst_cooldown = 0f32;
        }

        if !self.is_bursting && self.time_since_burst_cooldown < BURST_COOLDOWN {
          self.time_since_burst_cooldown += dt;
        }
      }
    }
  }
}