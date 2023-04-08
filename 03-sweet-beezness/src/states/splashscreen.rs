use std::time::Duration;

use bevy::{
  prelude::*,
  sprite::MaterialMesh2dBundle,
};

use super::AppState;

#[derive(Resource)]
pub struct SplashScreenData {
    title_entity: Entity,
    logo_entity: Entity,
    timer: Timer
}

pub fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>
) {
  let logo = commands.spawn(MaterialMesh2dBundle {
      mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
      material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
      transform: Transform::from_translation(Vec3::new(250., 0., 0.)),
      ..default()
  }).id();

  let font = asset_server.load("fonts/Roboto-Thin.ttf");
  let text_style = TextStyle {
      font: font.clone(),
      font_size: 60.0,
      color: Color::WHITE,
  };
  let text_alignment = TextAlignment::Center;

  let title = commands.spawn(
          Text2dBundle {
              text: Text::from_section("Sweet Beezness", text_style.clone())
                  .with_alignment(text_alignment),
              ..default()
          }
  ).id();

  commands.insert_resource(SplashScreenData {
      title_entity: title,
      logo_entity: logo,
      timer: Timer::new(Duration::new(5, 0), TimerMode::Once)
  });
}

pub fn update(
  time: Res<Time>,
  mut data: ResMut<SplashScreenData>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  if data.timer.tick(time.delta()).just_finished() {
      next_state.set(AppState::Tutorial);
  }
}

pub fn clean(
  mut commands: Commands,
  data: Res<SplashScreenData>
) {
  commands.entity(data.title_entity).despawn_recursive();
  commands.entity(data.logo_entity).despawn_recursive();
}