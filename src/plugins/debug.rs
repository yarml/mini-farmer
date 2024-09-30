use avian2d::prelude::PhysicsDebugPlugin;
use bevy::{
  app::{Plugin, Update},
  dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
  input::ButtonInput,
  prelude::{KeyCode, Res, ResMut},
};

fn toggle_debug_ui(
  input: Res<ButtonInput<KeyCode>>,
  mut debug_options: ResMut<UiDebugOptions>,
) {
  if input.just_pressed(KeyCode::F3) {
    debug_options.toggle();
  }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_plugins(DebugUiPlugin)
      .add_plugins(PhysicsDebugPlugin::default())
      .add_systems(Update, toggle_debug_ui);
  }
}
