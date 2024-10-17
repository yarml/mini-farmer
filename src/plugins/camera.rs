use std::f32::consts::PI;

use super::{daycycle::DayCycle, player::Player};
use bevy::{
  app::{App, Plugin, Startup, Update},
  math::Vec3,
  prelude::{
    Camera2d, Camera2dBundle, Commands, Component, Query, Res, Transform, With,
    Without,
  },
  time::Time,
  utils::default,
};
use bevy_light_2d::light::AmbientLight2d;

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
  let mut camera = Camera2dBundle::default();
  camera.projection.scale = 0.3;
  camera.transform.translation.x += 1280.0 / 4.0;
  camera.transform.translation.y += 720.0 / 4.0;
  commands.spawn((
    MainCamera,
    camera,
    AmbientLight2d {
      brightness: 0.01,
      ..default()
    },
  ));
}

fn update(
  time: Res<Time>,
  day: Res<DayCycle>,
  mut camera: Query<
    (&mut Transform, &mut AmbientLight2d),
    (With<Camera2d>, Without<Player>),
  >,
  player: Query<&Transform, With<Player>>,
) {
  let Ok((mut camera, mut sunlight)) = camera.get_single_mut() else {
    return;
  };

  sunlight.brightness = sunlight_brightness(day.daytime);

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

const A: f32 = 0.5;
const P: f32 = -0.643501108793;
const C: f32 = A;
const F: f32 = 1.14;

fn sunlight_brightness(daytime: f32) -> f32 {
  let t = daytime;
  A * f32::sin(2. * PI * F * t + P) + C
}
