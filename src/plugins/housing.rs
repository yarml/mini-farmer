use avian2d::prelude::{Collider, RigidBody};
use bevy::{
  app::{App, Plugin},
  prelude::Bundle,
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkIntCell};

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
  fn build(&self, app: &mut App) {
    app.register_ldtk_int_cell_for_layer::<HouseBundle>("worldmap", 3);
  }
}

#[derive(Bundle, LdtkIntCell)]
struct HouseBundle {
  body: RigidBody,
  collider: Collider,
}

impl Default for HouseBundle {
  fn default() -> Self {
    Self {
      body: RigidBody::Static,
      collider: Collider::rectangle(16., 16.),
    }
  }
}
