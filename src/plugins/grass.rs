use std::collections::HashMap;

use super::gen::WorldGen;
use bevy::{
  app::{App, Plugin, Update},
  prelude::{
    Bundle, Commands, Component, Entity, Query, Res, ResMut, Resource, With,
    Without,
  },
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, GridCoords, LdtkIntCell};

#[derive(Resource)]
pub struct ArabilityIndex {
  index: HashMap<(i32, i32), f32>,
}

#[derive(Default, Component)]
pub struct Grass;

#[derive(Component)]
struct Arable;

#[derive(Default, Bundle, LdtkIntCell)]
struct GrassBundle {
  grass: Grass,
}

fn gen(
  world_gen: Res<WorldGen>,
  mut index: ResMut<ArabilityIndex>,
  mut commands: Commands,
  grass_uninit: Query<(Entity, &GridCoords), (With<Grass>, Without<Arable>)>,
) {
  for (entity, coords) in &grass_uninit {
    let (x, y) = (coords.x, coords.y);
    let v = world_gen.at(x, y);
    index.index.insert((x, y), v);
    commands.entity(entity).insert(Arable);
  }
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<GrassBundle>("worldmap", 2)
      .add_systems(Update, gen)
      .insert_resource(ArabilityIndex::new());
  }
}

impl ArabilityIndex {
  fn new() -> Self {
    Self {
      index: HashMap::new(),
    }
  }
  pub fn get(&self, GridCoords { x, y }: GridCoords) -> Option<f32> {
    self.index.get(&(x, y)).copied()
  }
}
