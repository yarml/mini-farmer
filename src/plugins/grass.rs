use super::gen::WorldGen;
use bevy::{
  app::{App, Plugin, Update},
  prelude::{
    Bundle, Commands, Component, Entity, ParallelCommands, Query, Res, ResMut,
    Resource, With, Without,
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

pub enum FarmStage {
  Empty,
  Sprout,
  Vegetative,
  Ripening,
}

#[derive(Component)]
pub struct Farmland(pub FarmStage);

#[derive(Component)]
pub struct Watered;

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

fn apply_texture(
  mut q_grass: Query<
    (
      Entity,
      &mut TileTextureIndex,
      Option<&Farmland>,
      Option<&Watered>,
    ),
    With<Arability>,
  >,
  par_commands: ParallelCommands,
) {
  q_grass
    .par_iter_mut()
    .for_each(|(entity, mut index, farmland, watered)| {
      index.0 = if let Some(Farmland(stage)) = farmland {
        if index.0 != 31 && index.0 < 50 {
          par_commands.command_scope(|mut commands| {
            commands
              .entity(entity)
              .remove::<Farmland>()
              .remove::<Watered>();
          });
          index.0
        } else {
          let base = match stage {
            FarmStage::Empty => 80,
            FarmStage::Sprout => 120,
            FarmStage::Vegetative => 160,
            FarmStage::Ripening => 160,
          };
          if watered.is_some() {
            base + 5
          } else {
            base
          }
        }
      } else {
        if index.0 >= 50 {
          31
        } else {
          index.0
        }
      };
    });
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<GrassBundle>("worldmap", 2)
      .add_systems(Update, (gen, apply_texture))
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

impl FarmStage {
  pub fn next(&mut self) {
    *self = match self {
      FarmStage::Empty => FarmStage::Empty,
      FarmStage::Sprout => FarmStage::Vegetative,
      FarmStage::Vegetative => FarmStage::Ripening,
      FarmStage::Ripening => FarmStage::Ripening,
    }
  }
}
