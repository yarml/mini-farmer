use avian2d::{
  math::Scalar,
  prelude::{
    AngularDamping, AngularVelocity, Collider, Friction, Inertia,
    LinearDamping, LinearVelocity, Mass, Restitution, RigidBody,
  },
};
use bevy::prelude::{Bundle, Component};

#[derive(Default, Component)]
pub struct Acceleration(pub Scalar);

#[derive(Default, Bundle)]
pub struct PhysicsBundle {
  pub body: RigidBody,
  pub collider: Collider,
  pub acceleration: Acceleration,
  pub linear_velocity: LinearVelocity,
  pub angular_velocity: AngularVelocity,
  pub mass: Mass,
  pub inertia: Inertia,
  pub linear_damping: LinearDamping,
  pub angular_damping: AngularDamping,
  pub friction: Friction,
  pub restitution: Restitution,
}
