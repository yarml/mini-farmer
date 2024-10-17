use std::collections::{HashMap, HashSet};

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

use super::{grass::Grass, housing::House, road::Road};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
  Grass,
  Housing,
  Road,
  Water,
}

#[derive(Resource)]
pub struct WorldIndex {
  index: HashMap<(i32, i32), (Entity, TileType)>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<WaterBundle>(
        "worldmap",
        TileType::Water.index(),
      )
      .add_systems(Startup, setup_world)
      .add_systems(Update, (cache_water_coundaries, spawn_boundaries, cache_index))
      .insert_resource(LevelSelection::index(0))
      .insert_resource(WaterBoundaries {
        boundaries: HashSet::new(),
      })
      .insert_resource(WorldIndex {
        index: HashMap::new(),
      });
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

fn cache_index(
  mut ev_levels: EventReader<LevelEvent>,
  mut world_index: ResMut<WorldIndex>,
  q_water: Query<(Entity, &GridCoords), With<Water>>,
  q_grass: Query<(Entity, &GridCoords), With<Grass>>,
  q_housing: Query<(Entity, &GridCoords), With<House>>,
  q_road: Query<(Entity, &GridCoords), With<Road>>,
) {
  for ev in ev_levels.read() {
    if let LevelEvent::Spawned(_) = ev {
      world_index.index.clear();
      for (ent, coords) in &q_water {
        world_index.set(*coords, ent, TileType::Water);
      }
      for (ent, coords) in &q_grass {
        world_index.set(*coords, ent, TileType::Grass);
      }
      for (ent, coords) in &q_housing {
        world_index.set(*coords, ent, TileType::Housing);
      }
      for (ent, coords) in &q_road {
        world_index.set(*coords, ent, TileType::Road);
      }
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

impl TileType {
  pub fn index(&self) -> i32 {
    match self {
      TileType::Grass => 2,
      TileType::Housing => 3,
      TileType::Road => 4,
      TileType::Water => 1,
    }
  }
}

impl TryFrom<usize> for TileType {
  type Error = ();

  fn try_from(value: usize) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(Self::Water),
      2 => Ok(Self::Grass),
      3 => Ok(Self::Housing),
      4 => Ok(Self::Road),
      _ => Err(()),
    }
  }
}

impl WorldIndex {
  fn set(
    &mut self,
    GridCoords { x, y }: GridCoords,
    ent: Entity,
    typ: TileType,
  ) {
    self.index.insert((x, y), (ent, typ));
  }

  pub fn get(
    &self,
    GridCoords { x, y }: GridCoords,
  ) -> Option<(Entity, TileType)> {
    self.index.get(&(x, y)).copied()
  }
  pub fn get_entity(&self, coords: GridCoords) -> Option<Entity> {
    self.get(coords).map(|(ent, _)| ent)
  }
  pub fn get_type(&self, coords: GridCoords) -> Option<TileType> {
    self.get(coords).map(|(_, typ)| typ)
  }
}
