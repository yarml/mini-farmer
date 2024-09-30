use avian2d::{
  math::Scalar,
  prelude::{Collider, LinearDamping, LinearVelocity, Mass, RigidBody},
};
use bevy::{
  app::{App, Plugin, Update},
  input::ButtonInput,
  math::Vec2,
  prelude::{Bundle, Component, KeyCode, Query, Res, With},
  time::Time,
};

#[derive(Default, Component)]
pub struct PhysicsControls;

#[derive(Default, Component)]
pub struct Acceleration(pub Scalar);

#[derive(Default, Bundle)]
pub struct MovementBundle {
  pub acceleration: Acceleration,
  pub velocity: LinearVelocity,
  pub damping: LinearDamping,
  pub mass: Mass,
}

#[derive(Default, Bundle)]
pub struct PhysicsControlsBundle {
  pub controls: PhysicsControls,
  pub body: RigidBody,
  pub collider: Collider,
  pub movement: MovementBundle,
}

fn input(
  mut query: Query<(&mut LinearVelocity, &Acceleration), With<PhysicsControls>>,
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

  for (mut vel, acceleration) in &mut query {
    let delta_velocity =
      direction.normalize_or_zero() * acceleration.0 * time.delta_seconds();
    vel.0 += delta_velocity;
  }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, input);
  }
}
