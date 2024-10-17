use super::{
  daycycle::DayCycle,
  interface::Interface,
  world::{TileType, WorldIndex},
};
use avian2d::prelude::{Collider, RigidBody};
use bevy::{
  app::{App, Plugin, Update},
  color::Color,
  input::ButtonInput,
  log::info,
  prelude::{Bundle, Component, MouseButton, Res, ResMut},
  utils::default,
};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkIntCell};
use bevy_light_2d::light::PointLight2d;

fn try_sleep(
  interface: Res<Interface>,
  mouse: Res<ButtonInput<MouseButton>>,
  world_index: Res<WorldIndex>,
  mut day: ResMut<DayCycle>,
) {
  if mouse.just_pressed(MouseButton::Left) {
    let tile = interface.selected_tile(&world_index);
    info!("fefefefe");
    if let Some(typ) = tile {
      info!("dedede {typ:?}");
      if typ == TileType::Housing {
        day.sleep();
      }
    }
  }
}

#[derive(Component)]
pub struct House;

#[derive(Bundle, LdtkIntCell)]
struct HouseBundle {
  road: House,
  body: RigidBody,
  collider: Collider,
  light: PointLight2d,
}

impl Default for HouseBundle {
  fn default() -> Self {
    Self {
      road: House,
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

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ldtk_int_cell_for_layer::<HouseBundle>(
        "worldmap",
        TileType::Housing.index(),
      )
      .add_systems(Update, try_sleep);
  }
}
