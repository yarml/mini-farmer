use super::{
  camera::MainCamera,
  grass::{Arability, Farmland, Grass, GrassIndex},
};
use bevy::{
  app::{App, Plugin, Startup, Update},
  asset::AssetServer,
  color::{
    palettes::css::{BLACK, RED},
    Alpha,
  },
  input::keyboard::KeyboardInput,
  math::Vec2,
  prelude::{
    default, Camera, Commands, Component, Entity, EventReader, GlobalTransform,
    KeyCode, Query, Res, ResMut, Resource, TextBundle, Transform, Visibility,
    With,
  },
  sprite::{Sprite, SpriteBundle},
  text::{Text, TextSection, TextStyle},
  time::Time,
  window::{PrimaryWindow, Window},
};
use bevy_ecs_ldtk::GridCoords;
use core::f32;

#[derive(Resource)]
pub struct Interface {
  pub cursor: Vec2,
}

#[derive(Component)]
struct ArabilityText;

#[derive(Component)]
struct Selector;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
  commands.spawn((
    TextBundle::from_sections([
      TextSection::new(
        "Arability: ",
        TextStyle {
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
  interface: Res<Interface>,
  grass_index: Res<GrassIndex>,
  mut q_texts: Query<(&mut Text, &mut Visibility), With<ArabilityText>>,
  q_arability: Query<&Arability, With<Grass>>,
) {
  let (mut texts, mut visibility) = q_texts.single_mut();
  match interface.selected_grass(&grass_index).map(|grass_entity| {
    format!(
      "{v}%",
      v = (q_arability
        .get(grass_entity)
        .map_or(f32::NAN, |arability| arability.0)
        * 100.)
        .round()
    )
  }) {
    Some(value) => {
      texts.sections[1].value = value;
      *visibility = Visibility::Visible;
    }
    None => *visibility = Visibility::Hidden,
  }
}

fn update_selector(
  time: Res<Time>,
  interface: Res<Interface>,
  mut selector: Query<(&mut Transform, &mut Sprite), With<Selector>>,
) {
  let (mut transform, mut sprite) = selector.single_mut();
  let selector_pos = interface.selector_pos();
  transform.translation.x = selector_pos.x;
  transform.translation.y = selector_pos.y;
  transform.translation.z = 5.;

  sprite
    .color
    .set_alpha((time.elapsed_seconds() * 2.).sin().abs());
}

fn update_cursor(
  mut interface: ResMut<Interface>,
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
    interface.cursor = world_position;
  }
}

fn cultivate(
  interface: Res<Interface>,
  grass_index: Res<GrassIndex>,
  mut commands: Commands,
  mut kbd_evr: EventReader<KeyboardInput>,
) {
  for ev in kbd_evr.read() {
    if ev.state.is_pressed() && ev.key_code == KeyCode::Space {
      if let Some(mut grass_entity) = interface
        .selected_grass(&grass_index)
        .map(|selected_grass| commands.entity(selected_grass))
      {
        grass_entity.insert(Farmland);
      }
    }
  }
}

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Interface { cursor: default() })
      .add_systems(Startup, setup)
      .add_systems(
        Update,
        (update_cursor, update_arability, update_selector, cultivate),
      );
  }
}

impl Interface {
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
  pub fn selected_grass(&self, grass_index: &GrassIndex) -> Option<Entity> {
    let curs_coords = self.cursor_grid_coords();
    grass_index.get(curs_coords)
  }
}
