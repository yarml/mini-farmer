use bevy::{
  app::{App, Plugin, Update},
  math::Vec3,
  prelude::{
    Bundle, Component, EventReader, Query, Res, ResMut, Transform, With,
  },
  utils::default,
};
use bevy_ecs_ldtk::{
  app::LdtkIntCellAppExt, GridCoords, LdtkIntCell, LevelEvent,
};

use super::{gen::WorldGen, hud::Hud, player::Player};

#[derive(Default, Component)]
struct Arability(pub f32);

#[derive(Bundle, LdtkIntCell)]
struct GrassBundle {
  arability: Arability,
}

impl Default for GrassBundle {
  fn default() -> Self {
    Self {
      arability: default(),
    }
  }
}

fn gen_arability(
  world_gen: Res<WorldGen>,
  mut level_events: EventReader<LevelEvent>,
  mut grass: Query<(&GridCoords, &mut Arability)>,
) {
  for e in level_events.read() {
    if let LevelEvent::Spawned(_) = e {
      for (coords, mut arability) in &mut grass {
        let v = world_gen.at(coords.x, coords.y);
        println!("{}/{}: {}", coords.x, coords.y, v);
        arability.0 = v;
      }
    }
  }
}

fn update_arability(
  player: Query<&Transform, With<Player>>,
  grass: Query<(&Transform, &Arability)>,
  mut hud: ResMut<Hud>,
) {
  if let Ok(player) = player.get_single() {
    for (transform, arability) in &grass {
      let distance = transform.translation.distance(player.translation);
      if distance < 16. {
        hud.arability = Some(arability.0);
        hud.selector_pos = Some(
          transform.translation.clone()
            + Vec3 {
              x: 8.,
              y: 8.,
              z: 0.,
            },
        );
        return;
      }
    }
  }
  hud.arability = None;
  hud.selector_pos = None;
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<GrassBundle>("worldmap", 2)
      .add_systems(Update, gen_arability)
      .add_systems(Update, update_arability);
  }
}
