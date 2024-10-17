use super::{gen::WorldGen, world::TileType};
use bevy::{
  app::{App, Plugin, Update},
  prelude::{
    Bundle, Commands, Component, Entity, ParallelCommands, Query, Res, With,
    Without,
  },
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, GridCoords, LdtkIntCell};
use bevy_ecs_tilemap::tiles::TileTextureIndex;

#[derive(Default, Component)]
pub struct Grass;

#[derive(Component)]
pub struct Arability(pub f32);

#[derive(Clone, Copy, PartialEq)]
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
  mut commands: Commands,
  grass_uninit: Query<(Entity, &GridCoords), (With<Grass>, Without<Arability>)>,
) {
  for (entity, coords) in &grass_uninit {
    let (x, y) = (coords.x, coords.y);
    let v = world_gen.at(x, y);
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
          let base = stage.atlas_index();
          if watered.is_some() {
            (base + 5) as u32
          } else {
            base as u32
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
      .register_ldtk_int_cell_for_layer::<GrassBundle>(
        "worldmap",
        TileType::Grass.index(),
      )
      .add_systems(Update, (gen, apply_texture));
  }
}

impl FarmStage {
  pub fn plant(&mut self) {
    if *self == FarmStage::Empty {
      *self = FarmStage::Sprout;
    }
  }
  pub fn next(&mut self) {
    *self = match self {
      FarmStage::Empty => FarmStage::Empty,
      FarmStage::Sprout => FarmStage::Vegetative,
      FarmStage::Vegetative => FarmStage::Ripening,
      FarmStage::Ripening => FarmStage::Ripening,
    }
  }

  pub fn atlas_index(&self) -> usize {
    match self {
      FarmStage::Empty => 80,
      FarmStage::Sprout => 120,
      FarmStage::Vegetative => 160,
      FarmStage::Ripening => 160,
    }
  }
}
