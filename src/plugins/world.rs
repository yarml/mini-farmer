use bevy::{
  app::{App, Plugin, Startup},
  asset::AssetServer,
  prelude::{default, Commands, Res},
};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_world)
      .insert_resource(LevelSelection::index(0));
  }
}

fn setup_world(mut commands: Commands, server: Res<AssetServer>) {
  let world = server.load("levels.ldtk");
  commands.spawn(LdtkWorldBundle {
    ldtk_handle: world,
    ..default()
  });
}
