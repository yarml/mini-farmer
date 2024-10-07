use std::collections::HashSet;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{
  app::{App, Plugin, Startup, Update},
  asset::AssetServer,
  prelude::{
    default, Bundle, Children, Commands, Component, Entity, EventReader, Query,
    Res, ResMut, Resource, With, Without,
  },
};
use bevy_ecs_ldtk::{
  app::LdtkIntCellAppExt, GridCoords, LayerMetadata, LdtkIntCell,
  LdtkWorldBundle, LevelEvent, LevelSelection,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<WaterBundle>("worldmap", 1)
      .add_systems(Startup, setup_world)
      .add_systems(Update, cache_water_coundaries)
      .insert_resource(LevelSelection::index(0))
      .insert_resource(WaterBoundaries {
        boundaries: HashSet::new(),
      })
      .add_systems(Update, spawn_boundaries);
  }
}

fn setup_world(mut commands: Commands, server: Res<AssetServer>) {
  let world = server.load("levels.ldtk");
  commands.spawn(LdtkWorldBundle {
    ldtk_handle: world,
    ..default()
  });
}

#[derive(Default, Component)]
struct Water;

#[derive(Default, Bundle, LdtkIntCell)]
struct WaterBundle {
  water: Water,
}

#[derive(Resource)]
struct WaterBoundaries {
  boundaries: HashSet<GridCoords>,
}

fn cache_water_coundaries(
  mut level_water_boundaries: ResMut<WaterBoundaries>,
  mut level_events: EventReader<LevelEvent>,
  layers: Query<(&LayerMetadata, &Children)>,
  waters: Query<&GridCoords, With<Water>>,
  lands: Query<&GridCoords, Without<Water>>,
) {
  for e in level_events.read() {
    if let LevelEvent::Transformed(_) = e {
      let mut boundaries = HashSet::new();
      if let Some((_, children)) = layers
        .iter()
        .find(|(meta, _)| meta.identifier == "worldmap")
      {
        let mut water = HashSet::<GridCoords>::new();
        let mut land = HashSet::<GridCoords>::new();

        for &child in children.iter() {
          if let Ok(coords) = waters.get(child) {
            water.insert(coords.clone());
          } else if let Ok(coords) = lands.get(child) {
            land.insert(coords.clone());
          }
        }
        for l in &land {
          let neighbours = [
            GridCoords { x: l.x - 1, y: l.y },
            GridCoords { x: l.x + 1, y: l.y },
            GridCoords { x: l.x, y: l.y - 1 },
            GridCoords { x: l.x, y: l.y + 1 },
          ];
          for n in neighbours.iter() {
            if water.contains(n) {
              boundaries.insert(n.clone());
            }
          }
        }
      }
      level_water_boundaries.boundaries = boundaries;
    }
  }
}

fn spawn_boundaries(
  mut commands: Commands,
  level_water_boundaries: Res<WaterBoundaries>,
  waters: Query<(Entity, &GridCoords), (With<Water>, Without<Collider>)>,
) {
  for (w, coords) in &waters {
    if level_water_boundaries.boundaries.contains(coords) {
      commands
        .entity(w)
        .insert((Collider::rectangle(14., 14.), RigidBody::Static));
    }
  }
}
