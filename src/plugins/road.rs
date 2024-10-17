use super::world::TileType;
use bevy::{
  app::{App, Plugin},
  prelude::{Bundle, Component},
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkIntCell};

#[derive(Default, Component)]
pub struct Road;

#[derive(Bundle, Default, LdtkIntCell)]
struct RoadBundle {
  marker: Road,
}

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
  fn build(&self, app: &mut App) {
    app.register_ldtk_int_cell_for_layer::<RoadBundle>(
      "worldmap",
      TileType::Road.index(),
    );
  }
}
