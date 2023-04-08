use bevy::{
    prelude::*,
    // sprite::MaterialMesh2dBundle,
};

mod states;
use states::splashscreen;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_editor_pls::EditorPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_system(setup_camera.on_startup())
    .add_state::<states::AppState>()
    .add_system(splashscreen::setup.in_schedule(OnEnter(states::AppState::SplashScreen)))
    .add_system(splashscreen::update.in_set(OnUpdate(states::AppState::SplashScreen)))
    .add_system(splashscreen::clean.in_schedule(OnExit(states::AppState::SplashScreen)))
    .run();
}

fn setup_camera(
    mut commands: Commands
) {
    let cam = Camera2dBundle::default();

    commands.spawn(cam);
}


// fn keyboard_debug(
//     input: Res<Input<KeyCode>>
// ) {
//     if input.just_pressed(KeyCode::A) {
//         println!("A");
//     }
// }