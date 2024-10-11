use super::player::Player;
use bevy::{
  app::{App, Plugin, Startup, Update},
  math::Vec3,
  prelude::{
    Camera2d, Camera2dBundle, Commands, Component, Query, Res, Transform, With,
    Without,
  },
  time::Time,
};

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
  let mut camera = Camera2dBundle::default();
  camera.projection.scale = 0.3;
  camera.transform.translation.x += 1280.0 / 4.0;
  camera.transform.translation.y += 720.0 / 4.0;
  commands.spawn((MainCamera, camera));
}

fn update(
  mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
  player: Query<&Transform, With<Player>>,
  time: Res<Time>,
) {
  let Ok(mut camera) = camera.get_single_mut() else {
    return;
  };
  let Ok(player) = player.get_single() else {
    return;
  };

  let Vec3 { x, y, .. } = player.translation;
  let direction = Vec3::new(x, y, camera.translation.z);

  camera.translation = camera
    .translation
    .lerp(direction, time.delta_seconds() * 1.);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_camera)
      .add_systems(Update, update);
  }
}
