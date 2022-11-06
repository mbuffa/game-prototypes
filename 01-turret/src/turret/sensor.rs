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

    let angle1 = (self.angle + 30.0) * PI / 180.0;
    draw_line(
      self.x, 
      self.y, 
      self.x + (self.radius * angle1.cos()),
      self.y + (self.radius * angle1.sin()),
      1f32,
      RED
    );

    let angle2 = (self.angle - 30.0) * PI / 180.0;
    draw_line(
      self.x, 
      self.y, 
      self.x + (self.radius * angle2.cos()),
      self.y + (self.radius * angle2.sin()),
      1f32,
      RED
    );
  }

  pub fn update(&mut self) {
    let dt = get_frame_time();

    if self.angle <= self.base_angle + LEFT_BOUNDARY {
      self.rot_direction = 1f32;
    }

    if self.angle >= self.base_angle + RIGHT_BOUNDARY {
      self.rot_direction = -1f32;
    }

    self.angle += self.rot_direction * ROT_VELOCITY * dt;
  }
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