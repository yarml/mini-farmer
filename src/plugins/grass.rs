use super::gen::WorldGen;
use bevy::{
  app::{App, Plugin, PostUpdate, Update},
  prelude::{
    Added, Bundle, Commands, Component, Entity, Query, RemovedComponents, Res,
    ResMut, Resource, With, Without,
  },
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, GridCoords, LdtkIntCell};
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use std::collections::HashMap;

#[derive(Resource)]
pub struct GrassIndex {
  index: HashMap<(i32, i32), Entity>,
}

#[derive(Default, Component)]
pub struct Grass;

#[derive(Component)]
pub struct Arability(pub f32);

#[derive(Component)]
pub struct Farmland;

#[derive(Default, Bundle, LdtkIntCell)]
struct GrassBundle {
  grass: Grass,
}

fn gen(
  world_gen: Res<WorldGen>,
  mut index: ResMut<GrassIndex>,
  mut commands: Commands,
  grass_uninit: Query<(Entity, &GridCoords), (With<Grass>, Without<Arability>)>,
) {
  for (entity, coords) in &grass_uninit {
    let (x, y) = (coords.x, coords.y);
    let v = world_gen.at(x, y);
    index.index.insert((x, y), entity);
    commands.entity(entity).insert(Arability(v));
  }
}

fn apply_farmland_texture(
  mut q_grass: Query<&mut TileTextureIndex, (With<Arability>, Added<Farmland>)>,
) {
  q_grass.par_iter_mut().for_each(|mut index| index.0 += 6);
}

fn apply_grass_texture(
  mut removed: RemovedComponents<Farmland>,
  mut q_grass: Query<
    &mut TileTextureIndex,
    (With<Arability>, Without<Farmland>),
  >,
) {
  for entity in removed.read() {
    if let Some(mut index) = q_grass.get_mut(entity).ok() {
      index.0 -= 6;
    }
  }
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<GrassBundle>("worldmap", 2)
      .add_systems(Update, (gen, apply_farmland_texture))
      .add_systems(PostUpdate, apply_grass_texture)
      .insert_resource(GrassIndex::new());
  }
}

impl GrassIndex {
  fn new() -> Self {
    Self {
      index: HashMap::new(),
    }
  }
  pub fn get(&self, GridCoords { x, y }: GridCoords) -> Option<Entity> {
    self.index.get(&(x, y)).copied()
  }
}
