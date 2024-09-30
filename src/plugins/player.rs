use super::controls::{
  Acceleration, MovementBundle, PhysicsControls, PhysicsControlsBundle,
};
use avian2d::prelude::{Collider, LinearDamping, Mass, RigidBody};
use bevy::{
  app::{App, Plugin},
  prelude::{Bundle, Component},
};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("player");
  }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
  marker: Player,
  controls: PhysicsControlsBundle,
  #[sprite_sheet_bundle]
  sprite: LdtkSpriteSheetBundle,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    Self {
      marker: Default::default(),
      sprite: Default::default(),
      controls: PhysicsControlsBundle {
        controls: PhysicsControls,
        body: RigidBody::Dynamic,
        collider: Collider::circle(8.),
        movement: MovementBundle {
          acceleration: Acceleration(512.0),
          velocity: Default::default(),
          damping: LinearDamping(8.),
          mass: Mass(1.),
        },
      },
    }
  }
}
