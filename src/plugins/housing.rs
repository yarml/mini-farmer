use avian2d::prelude::{Collider, RigidBody};
use bevy::{
  app::{App, Plugin},
  color::Color,
  prelude::Bundle,
  utils::default,
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkIntCell};
use bevy_light_2d::light::PointLight2d;

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
  light: PointLight2d,
}

impl Default for HouseBundle {
  fn default() -> Self {
    Self {
      body: RigidBody::Static,
      collider: Collider::rectangle(16., 16.),
      light: PointLight2d {
        color: Color::linear_rgb(1., 0.654901961, 0.223529412),
        radius: 32.,
        ..default()
      },
    }
  }
}
