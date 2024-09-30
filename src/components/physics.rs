use avian2d::prelude::{AngularVelocity, Inertia, LinearVelocity, Mass};
use bevy::prelude::Bundle;

#[derive(Default, Bundle)]
pub struct PhysicsBundle {
  pub linear_velocity: LinearVelocity,
  pub angular_velocity: AngularVelocity,
  pub mass: Mass,
  pub inertia: Inertia,
}
