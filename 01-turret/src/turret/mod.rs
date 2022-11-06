mod base;
mod sensor;

pub struct Turret {
  x: f32,
  y: f32,
  angle: f32,
  base: base::Base,
  sensor: sensor::Sensor
}

impl Turret {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    Self {
      x: x,
      y: y,
      angle: angle,
      base: base::Base::new(x, y, angle),
      sensor: sensor::Sensor::new(x, y, angle)
    }
  }

  pub fn draw(& self) {
    self.base.draw();
    self.sensor.draw();
  }

  pub fn update(&mut self) {
    self.sensor.update();
  }
}
