use std::f32::consts::PI;

use macroquad::{prelude::*};

// Rotation direction. -1 means it'll initially go to the left.
const ROT_DIRECTION: f32 = -1f32;

// Rotation velocity, in degrees per second.
const ROT_VELOCITY: f32 = 30f32;

// Left boundary, in degrees, relative to base angle.
const LEFT_BOUNDARY: f32 = -60f32;

// Right boundary, in degrees, relative to base angle.
const RIGHT_BOUNDARY: f32 = 60f32;

pub struct Sensor {
  x: f32,
  y: f32,
  radius: f32,
  base_angle: f32,
  angle: f32,
  rot_direction: f32
}

impl Sensor {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    Self {
      x: x, y: y,
      radius: 60f32,
      base_angle: angle,
      angle: angle,
      rot_direction: ROT_DIRECTION
    }
  }

  pub fn draw(&self) {
    // draw_circle_lines(self.x, self.y, self.radius, 1f32, DARKGRAY);
    draw_arc_lines(self.x, self.y, self.radius, self.angle - 30f32, 1f32, DARKGRAY);

    let rangle = (self.angle + 30f32) * PI / 180.0;
    let langle = (self.angle - 30f32) * PI / 180.0;

    draw_line(
      self.x, 
      self.y, 
      self.x + (self.radius * rangle.cos()),
      self.y + (self.radius * rangle.sin()),
      1f32,
      RED
    );

    draw_line(
      self.x, 
      self.y, 
      self.x + (self.radius * langle.cos()),
      self.y + (self.radius * langle.sin()),
      1f32,
      RED
    );

    // draw_triangle_lines(
    //   Vec2::new(self.x, self.y),
    //   Vec2::new(self.x + ((self.radius + 10f32) * rangle.cos()), self.y + ((self.radius + 10f32) * rangle.sin())),
    //   Vec2::new(self.x + ((self.radius + 10f32) * langle.cos()), self.y + ((self.radius + 10f32) * langle.sin())),
    //   1f32,
    //   BLUE
    // );
  }

  pub fn update(&mut self, elapsed: f32) {
    if self.angle <= self.base_angle + LEFT_BOUNDARY {
      self.rot_direction = 1f32;
    }

    if self.angle >= self.base_angle + RIGHT_BOUNDARY {
      self.rot_direction = -1f32;
    }

    self.angle += self.rot_direction * ROT_VELOCITY * elapsed;

    // let position = Vec2::from(mouse_position());

    // let seen = self.sees(position);

    // if seen {
    //   println!("I saw the mouse!");
    // } else {
    //   println!("Beep...");
    // }
  }

  pub fn sees(&self, other: Vec2) -> bool {
    // https://www.geeksforgeeks.org/check-whether-a-given-point-lies-inside-a-triangle-or-not/
    // Let the coordinates of three corners be (x1, y1), (x2, y2) and (x3, y3). And coordinates of the given point P be (x, y)
    // Calculate area of the given triangle, i.e., area of the triangle ABC in the above diagram. 
    // Area A = [ x1(y2 – y3) + x2(y3 – y1) + x3(y1-y2)]/2 
    // Calculate area of the vision triangle (ABC).
    // Calculate 3 areas: PAC, PBC, PAB.
    // Returns true if A = A1 + A2 + A3
    
    let rangle = (self.angle + 30f32) * PI / 180.0;
    let langle = (self.angle - 30f32) * PI / 180.0;
    
    let x1 = self.x;
    let y1 = self.y;
    let x2 = self.x + ((self.radius + 10f32) * rangle.cos());
    let y2 = self.y + ((self.radius + 10f32) * rangle.sin());
    let x3 = self.x + ((self.radius + 10f32) * langle.cos());
    let y3 = self.y + ((self.radius + 10f32) * langle.sin());
    
    let a: f32 = triangle_area(x1, y1, x2, y2, x3, y3);
    let a1: f32 = triangle_area(other.x, other.y, x2, y2, x3, y3);
    let a2: f32 = triangle_area(x1, y1, other.x, other.y, x3, y3);
    let a3: f32 = triangle_area(x1, y1, x2, y2, other.x, other.y);

    a == a1 + a2 + a3
  }
}

fn triangle_area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
  ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
}

fn draw_arc_lines(
    x: f32,
    y: f32,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
) {
    let rot = rotation.to_radians();
    let full_circle_sides = 30 as u8;
    let half_circle_sides = 5;

    for i in 0..half_circle_sides {
        let rx = (i as f32 / full_circle_sides as f32 * std::f32::consts::PI * 2. + rot).cos();
        let ry = (i as f32 / full_circle_sides as f32 * std::f32::consts::PI * 2. + rot).sin();

        let p0 = vec2(x + radius * rx, y + radius * ry);

        let rx = ((i + 1) as f32 / full_circle_sides as f32 * std::f32::consts::PI * 2. + rot).cos();
        let ry = ((i + 1) as f32 / full_circle_sides as f32 * std::f32::consts::PI * 2. + rot).sin();

        let p1 = vec2(x + radius * rx, y + radius * ry);

        draw_line(p0.x, p0.y, p1.x, p1.y, thickness, color);
    }
}
