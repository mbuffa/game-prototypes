use bevy::prelude::*;

#[derive(Component)]
struct Dino;

#[derive(Component, PartialEq)]
enum DinoState {
    Running,
    Jumping,
    Falling
}

#[derive(Component)]
struct JumpedAt(u128);

// TODO: Implement smooth jumping:
// - Base jumps with a velocity added to the sprite y axis. 
// - The longer we hit Space, the higher we jump.
//   - While we're jumping, keeping space pressed should increase our velocity (less and less over time).
// - Potentially use a small physics engine and some ground to avoid falling through the floor.

// TODO: Write better scheduling:
// - Extract DinoState as a global resource
// - Only run state updates when we're in a correct state:
//  - Running -> Jumping
//  - Jumping -> Falling
//  - Falling -> Running

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_dino_sprite)
        .add_system(maybe_start_jumping)
        .add_system(maybe_start_falling)
        .add_system(maybe_resume_running)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("dino.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Dino,
        DinoState::Running,
        JumpedAt(0)
    ));
}

fn update_dino_sprite(
    time: Res<Time>,
    mut query: Query<(&DinoState, &JumpedAt, &mut Transform), With<Dino>>
) {
    for (dino_state, jumped_at, mut transform) in query.iter_mut() {
        match dino_state {
            DinoState::Running => {
                // transform.translation.x += time.delta_seconds() * 100.;
            },
            DinoState::Jumping => {
                // transform.translation.y += time.delta_seconds() * 200.;
                transform.translation.y += ((time.elapsed().as_millis() - jumped_at.0) * 2) as f32;
            },
            DinoState::Falling => {
                // transform.translation.y -= time.delta_seconds() * 200.;
                transform.translation.y -= ((time.elapsed().as_millis() - jumped_at.0) * 2) as f32;
            }
        }
    }
}

fn maybe_start_jumping(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut DinoState, &mut JumpedAt), With<Dino>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut dino_state, mut jumped_at) in query.iter_mut() {
            if *dino_state == DinoState::Running {
                println!("Jump!");
                *dino_state = DinoState::Jumping;
                *jumped_at = JumpedAt(time.elapsed().as_millis());
            }
        }
    }
}

fn maybe_start_falling(
    time: Res<Time>,
    mut query: Query<(&mut DinoState, &mut JumpedAt), With<Dino>>,
) {
    for (mut dino_state, jumped_at) in query.iter_mut() {
        if *dino_state == DinoState::Jumping {
            if time.elapsed().as_millis() - jumped_at.0 > 750 {
                println!("Fall!");
                *dino_state = DinoState::Falling;
            }
        }
    }
}

fn maybe_resume_running(
    time: Res<Time>,
    mut query: Query<(&mut DinoState, &mut JumpedAt), With<Dino>>
) {
    for (mut dino_state, mut jumped_at) in query.iter_mut() {
        if *dino_state == DinoState::Falling {
            if time.elapsed().as_millis() - jumped_at.0 > 1500 {
                println!("Run!");
                *dino_state = DinoState::Running;
                *jumped_at = JumpedAt(0);
            }
        }
    }
}
