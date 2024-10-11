use super::{camera::MainCamera, grass::ArabilityIndex};
use bevy::{
  app::{App, Plugin, Startup, Update},
  asset::AssetServer,
  color::{
    palettes::css::{BLACK, RED},
    Alpha,
  },
  math::Vec2,
  prelude::{
    default, Camera, Commands, Component, GlobalTransform, Query, Res, ResMut,
    Resource, TextBundle, Transform, Visibility, With,
  },
  sprite::{Sprite, SpriteBundle},
  text::{Text, TextSection, TextStyle},
  time::Time,
  window::{PrimaryWindow, Window},
};
use bevy_ecs_ldtk::GridCoords;

#[derive(Resource)]
pub struct Hud {
  pub cursor: Vec2,
}

#[derive(Component)]
struct ArabilityText;

#[derive(Component)]
struct Selector;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
  commands.spawn((
    // Create a TextBundle that has a Text with a list of sections.
    TextBundle::from_sections([
      TextSection::new(
        "Arability: ",
        TextStyle {
          // This font is loaded and will be used instead of the default font.
          font: server.load("pixelify.ttf"),
          font_size: 60.0,
          color: BLACK.into(),
          ..default()
        },
      ),
      TextSection::from_style(TextStyle {
        font: server.load("pixelify.ttf"),
        font_size: 60.0,
        color: RED.into(),
      }),
    ]),
    ArabilityText,
  ));

  commands.spawn((
    Selector,
    SpriteBundle {
      texture: server.load("selector.png"),
      ..default()
    },
  ));
}

fn update_arability(
  hud: Res<Hud>,
  arability_index: Res<ArabilityIndex>,
  mut q_texts: Query<(&mut Text, &mut Visibility), With<ArabilityText>>,
) {
  let cursor_grid = hud.cursor_grid_coords();
  let (mut texts, mut visibility) = q_texts.single_mut();
  match arability_index
    .get(cursor_grid)
    .map(|arability| format!("{arability:.2}"))
  {
    Some(value) => {
      texts.sections[1].value = value;
      *visibility = Visibility::Visible;
    }
    None => *visibility = Visibility::Hidden,
  }
}

fn update_selector(
  time: Res<Time>,
  hud: Res<Hud>,
  mut selector: Query<(&mut Transform, &mut Sprite), With<Selector>>,
) {
  let (mut transform, mut sprite) = selector.single_mut();
  let selector_pos = hud.selector_pos();
  transform.translation.x = selector_pos.x;
  transform.translation.y = selector_pos.y;
  transform.translation.z = 5.;

  sprite
    .color
    .set_alpha((time.elapsed_seconds() * 2.).sin().abs());
}

fn update_cursor(
  mut hud: ResMut<Hud>,
  q_window: Query<&Window, With<PrimaryWindow>>,
  q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  let (camera, camera_transform) = q_camera.single();
  let window = q_window.single();

  if let Some(world_position) = window
    .cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate())
  {
    hud.cursor = world_position;
  }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Hud { cursor: default() })
      .add_systems(Startup, setup)
      .add_systems(Update, (update_cursor, update_arability, update_selector));
  }
}

impl Hud {
  pub fn cursor_grid_coords(&self) -> GridCoords {
    let v = ((self.cursor - 8.) / 16.).round();
    GridCoords {
      x: v.x as i32,
      y: v.y as i32,
    }
  }
  pub fn selector_pos(&self) -> Vec2 {
    (self.cursor / 16.).ceil() * 16. - 8.
  }
}
