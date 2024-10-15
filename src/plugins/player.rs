use std::time::Duration;

use super::controls::{Direction, PhysicsControlsBundle};
use crate::components::physics::{Acceleration, PhysicsBundle};
use avian2d::prelude::{
  AngularVelocity, Collider, LinearDamping, Mass, PhysicsSchedule,
  PhysicsStepSet, Restitution, RigidBody,
};
use bevy::{
  app::{App, Plugin, Update},
  input::ButtonInput,
  math::Vec3,
  prelude::{
    default, Bundle, Component, IntoSystemConfigs, KeyCode, Query, Res,
    Transform, With,
  },
  sprite::{SpriteBundle, TextureAtlas},
  time::{Time, Timer, TimerMode},
};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle};

const ANIM_FPS: f32 = 12.;

#[derive(Default, Component)]
pub struct Player;

#[derive(Component)]
struct AnimationConfig {
  phase: usize,
  timer: Timer,
}

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
  marker: Player,
  controls: PhysicsControlsBundle,
  #[sprite_sheet_bundle]
  sprite: LdtkSpriteSheetBundle,
  anim: AnimationConfig,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    Self {
      marker: default(),
      sprite: LdtkSpriteSheetBundle {
        sprite_bundle: SpriteBundle {
          transform: Transform {
            translation: Vec3 {
              z: 10.,
              ..default()
            },
            ..default()
          },
          ..default()
        },
        texture_atlas: default(),
      },
      controls: PhysicsControlsBundle {
        physics: PhysicsBundle {
          body: RigidBody::Dynamic,
          collider: Collider::rectangle(12., 1.),
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
      anim: AnimationConfig {
        phase: 0,
        timer: Timer::new(
          Duration::from_secs_f32(1. / ANIM_FPS),
          TimerMode::Repeating,
        ),
      },
    }
  }
}

fn apply_texture(
  time: Res<Time>,
  mut q_player: Query<
    (&mut TextureAtlas, &Direction, &mut AnimationConfig),
    With<Player>,
  >,
  kbd: Res<ButtonInput<KeyCode>>,
) {
  for (mut atlas, dir, mut anim) in &mut q_player {
    anim.timer.tick(time.delta());
    let moving = kbd.pressed(KeyCode::KeyA)
      || kbd.pressed(KeyCode::KeyD)
      || kbd.pressed(KeyCode::KeyW)
      || kbd.pressed(KeyCode::KeyS);
    if anim.timer.just_finished() {
      anim.phase += 4;
      anim.phase %= 12;
    }
    if !moving {
      anim.phase = 0;
    }
    atlas.index = dir.atlas_index() + anim.phase;
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
      .add_systems(Update, apply_texture)
      .add_systems(
        PhysicsSchedule,
        cancel_angular_change.in_set(PhysicsStepSet::First),
      );
  }
}
