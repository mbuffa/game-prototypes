use macroquad::{prelude::*};

mod turret;
mod spawner;

// const FACING_EAST: f32 = 0f32;
// const FACING_NORTH: f32 = -90f32;
const FACING_WEST: f32 = -180f32;
// const FACING_SOUTH: f32 = 90f32;

const MISSILE_VELOCITY: f32 = 200f32;

#[macroquad::main("Turret")]
async fn main() {
  let x = screen_width() / 2f32;
  let y = screen_height() / 2f32;
  let angle = FACING_WEST;

  let mut turret = turret::Turret::new(x, y, angle);
  let mut spawner = spawner::Spawner::new();

  // FIXME: Missiles are misplaced!
  // println!("Turret coords are {} {}", x, y);
  // println!("Turret coords are {} {}", turret.get_cannon_end_x(),
  // turret.get_cannon_end_y());

  loop {
    let elapsed = get_frame_time();

    if is_key_down(KeyCode::Space) {
      spawner.spawn_missile(
        turret.get_cannon_end_x(),
        turret.get_cannon_end_y(),
        turret.get_cannon_angle(),
        MISSILE_VELOCITY
      );
    }

    spawner.update(elapsed);
    turret.update(elapsed);

    clear_background(BLACK);
    spawner.draw();
    turret.draw();
    next_frame().await
  }
  
}
