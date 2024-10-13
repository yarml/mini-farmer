use crate::components::physics::{Acceleration, PhysicsBundle};
use avian2d::prelude::LinearVelocity;
use bevy::{
  app::{App, Plugin, Update},
  input::ButtonInput,
  math::Vec2,
  prelude::{Bundle, Component, KeyCode, Query, Res, With},
  time::Time,
};

#[derive(Default, Component)]
pub struct PhysicsControls;

#[derive(Default, Bundle)]
pub struct PhysicsControlsBundle {
  pub controls: PhysicsControls,
  pub physics: PhysicsBundle,
  pub direction: Direction,
}

#[derive(Default, Component)]
pub enum Direction {
  Up,
  #[default]
  Down,
  Left,
  Right,
}

fn input(
  mut q_target: Query<
    (&mut LinearVelocity, &Acceleration, &mut Direction),
    With<PhysicsControls>,
  >,
  time: Res<Time>,
  kbd: Res<ButtonInput<KeyCode>>,
) {
  let mut direction = Vec2::ZERO;

  if kbd.pressed(KeyCode::KeyA) {
    direction.x -= 1.;
  }
  if kbd.pressed(KeyCode::KeyD) {
    direction.x += 1.;
  }
  if kbd.pressed(KeyCode::KeyW) {
    direction.y += 1.;
  }
  if kbd.pressed(KeyCode::KeyS) {
    direction.y -= 1.;
  }

  for (mut vel, acceleration, mut dir) in &mut q_target {
    *dir = Direction::from(direction);
    let delta_velocity =
      direction.normalize_or_zero() * acceleration.0 * time.delta_seconds();
    vel.0 += delta_velocity;
  }
}

impl Direction {
  pub const fn atlas_index(&self) -> usize {
    match self {
      Direction::Up => 2,
      Direction::Down => 1,
      Direction::Left => 0,
      Direction::Right => 3,
    }
  }
}

impl From<Vec2> for Direction {
  fn from(value: Vec2) -> Self {
    if value.x < 0. {
      Direction::Left
    } else if value.x > 0. {
      Direction::Right
    } else if value.y > 0. {
      Direction::Up
    } else {
      Direction::Down
    }
  }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, input);
  }
}
