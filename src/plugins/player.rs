use super::controls::PhysicsControlsBundle;
use crate::components::physics::{Acceleration, PhysicsBundle};
use avian2d::prelude::{
  AngularVelocity, Collider, LinearDamping, Mass, Restitution, RigidBody,
};
use bevy::{
  app::{App, Plugin, Update},
  prelude::{Bundle, Component, Query, Transform, With},
  utils::default,
};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle};

#[derive(Default, Component)]
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
        physics: PhysicsBundle {
          body: RigidBody::Dynamic,
          collider: Collider::rectangle(12., 12.),
          acceleration: Acceleration(512.0),
          linear_damping: LinearDamping(8.),
          mass: Mass(60.),
          restitution: Restitution {
            coefficient: 0.,
            ..default()
          },

          ..default()
        },
        ..default()
      },
    }
  }
}

fn cancel_angular_change(
  mut query: Query<(&mut Transform, &mut AngularVelocity), With<Player>>,
) {
  if let Ok((mut transform, mut ang_vel)) = query.get_single_mut() {
    ang_vel.0 = 0.;
    transform.rotation = Default::default();
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_entity_for_layer::<PlayerBundle>("entities", "player")
      .add_systems(Update, cancel_angular_change);
  }
}
