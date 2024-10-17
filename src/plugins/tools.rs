use bevy::{
  app::{App, Plugin},
  asset::{AssetServer, Handle},
  ecs::system::EntityCommands,
  prelude::{Image, Mut, Resource},
};

use super::grass::{FarmStage, Farmland, Watered};

#[derive(Resource)]
pub enum Tool {
  Cultivate,
  Plant,
  Water,
  Harvest,
}

impl Tool {
  pub fn name(&self) -> &'static str {
    match self {
      Tool::Cultivate => "cultivate",
      Tool::Plant => "plant",
      Tool::Water => "water",
      Tool::Harvest => "harvest",
    }
  }

  pub fn texture(&self, server: &AssetServer) -> Handle<Image> {
    server.load(&format!("ui/{}.png", self.name()))
  }

  pub fn cycle(&mut self) {
    *self = match self {
      Tool::Cultivate => Tool::Plant,
      Tool::Plant => Tool::Water,
      Tool::Water => Tool::Harvest,
      Tool::Harvest => Tool::Cultivate,
    };
  }
  pub fn rev_cycle(&mut self) {
    *self = match self {
      Tool::Cultivate => Tool::Harvest,
      Tool::Plant => Tool::Cultivate,
      Tool::Water => Tool::Plant,
      Tool::Harvest => Tool::Water,
    };
  }

  pub fn activate(
    &self,
    mut target: EntityCommands,
    farmland: Option<Mut<'_, Farmland>>,
  ) {
    match self {
      Tool::Cultivate => {
        target.insert(Farmland(FarmStage::Empty));
      }
      Tool::Plant => {
        if let Some(mut farmland) = farmland {
          farmland.0.plant();
        }
      }
      Tool::Water => {
        target.insert(Watered);
      }
      Tool::Harvest => todo!(),
    };
  }

  pub fn deactivate(&self, mut target: EntityCommands) {
    match self {
      Tool::Cultivate => {
        target.remove::<Farmland>();
      }
      _ => {}
    };
  }
}

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(Tool::Cultivate);
  }
}
