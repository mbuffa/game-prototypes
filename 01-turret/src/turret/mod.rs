mod base;
mod gun;
mod sensor;

enum State {
  LookingForTarget,
  TargetAcquired
}

pub struct Turret {
  state: State,
  base: base::Base,
  gun: gun::Gun,
  sensor: sensor::Sensor
}

impl Turret {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    Self {
      state: State::LookingForTarget,
      base: base::Base::new(x, y),
      gun: gun::Gun::new(x, y, angle),
      sensor: sensor::Sensor::new(x, y, angle)
    }
  }

  pub fn draw(& self) {
    self.base.draw();
    self.gun.draw();
    self.sensor.draw();
  }

  pub fn update(&mut self, elapsed:f32) {
    self.gun.update(elapsed);
    self.sensor.update(elapsed);
  }

  pub fn get_cannon_angle(&self) -> f32 {
    self.gun.get_angle()
  }

  pub fn get_cannon_end_x(&self) -> f32 {
    self.gun.get_gun_end_x()
  }

  pub fn get_cannon_end_y(&self) -> f32 {
    self.gun.get_gun_end_y()
  }
}
