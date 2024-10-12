mod components;
mod plugins;

use avian2d::{prelude::Gravity, PhysicsPlugins};
use bevy::{
  app::{App, AppExit, PluginGroup},
  math::Vec2,
  prelude::ImagePlugin,
  utils::default,
  window::{Window, WindowPlugin},
  DefaultPlugins,
};
use bevy_ecs_ldtk::LdtkPlugin;
use plugins::{
  camera::CameraPlugin, controls::ControlsPlugin, gen::WorldGenPlugin, grass::GrassPlugin, housing::HousingPlugin, interface::InterfacePlugin, player::PlayerPlugin, tools::ToolsPlugin, world::WorldPlugin
};

fn main() -> AppExit {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(
      WindowPlugin {
        primary_window: Some(Window {
          title: String::from("Farmer"),
          name: Some(String::from("yarml.farmer")),
          resizable: false,
          ..default()
        }),
        ..default()
      },
    ))
    //.add_plugins(DebugPlugin)
    .add_plugins(LdtkPlugin)
    .add_plugins(PhysicsPlugins::default().with_length_unit(16.0))
    .add_plugins(PlayerPlugin)
    .add_plugins(ToolsPlugin)
    .add_plugins(ControlsPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(WorldPlugin)
    .add_plugins(GrassPlugin)
    .add_plugins(HousingPlugin)
    .add_plugins(WorldGenPlugin)
    .add_plugins(InterfacePlugin)
    .insert_resource(Gravity(Vec2::ZERO))
    .run()
}
