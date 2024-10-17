use super::{
  camera::MainCamera,
  grass::{Arability, Farmland, Grass},
  tools::Tool,
  world::{TileType, WorldIndex},
};
use bevy::{
  app::{App, Plugin, Startup, Update},
  asset::AssetServer,
  color::{
    palettes::css::{BLACK, RED},
    Alpha,
  },
  input::ButtonInput,
  math::Vec2,
  prelude::{
    default, BuildChildren, Camera, Commands, Component, Entity,
    GlobalTransform, KeyCode, MouseButton, NodeBundle, Query, Res, ResMut,
    Resource, TextBundle, Transform, Visibility, With,
  },
  sprite::{Sprite, SpriteBundle},
  text::{Text, TextSection, TextStyle},
  time::Time,
  ui::{Display, FlexDirection, JustifyContent, Style, UiImage, UiRect, Val},
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
struct ToolIcon;

#[derive(Component)]
struct Selector;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::SpaceBetween,
        display: Display::Flex,
        padding: UiRect::all(Val::Percent(1.)),
        flex_direction: FlexDirection::Column,
        ..default()
      },
      ..default()
    })
    .with_children(|root| {
      root.spawn((
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

      root.spawn((
        NodeBundle {
          style: Style {
            width: Val::Px(48.),
            height: Val::Px(48.),
            ..default()
          },
          ..default()
        },
        UiImage::new(server.load("ui/cultivate.png")),
        ToolIcon,
      ));
    });

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
  world_index: Res<WorldIndex>,
  mut q_texts: Query<(&mut Text, &mut Visibility), With<ArabilityText>>,
  q_arability: Query<&Arability, With<Grass>>,
) {
  let (mut texts, mut visibility) = q_texts.single_mut();
  match interface.selected_grass(&world_index).map(|grass_entity| {
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

fn tool_activate(
  tool: Res<Tool>,
  interface: Res<Interface>,
  mouse: Res<ButtonInput<MouseButton>>,
  world_index: Res<WorldIndex>,
  mut q_grass: Query<Option<&mut Farmland>, With<Grass>>,
  mut commands: Commands,
) {
  if mouse.pressed(MouseButton::Left) {
    if let Some((commands, farmland)) = interface
      .selected_grass(&world_index)
      .map(|selected_grass| {
        (
          commands.entity(selected_grass),
          q_grass.get_mut(selected_grass).ok(),
        )
      })
    {
      let Some(farmland) = farmland else {
        return;
      };
      tool.activate(commands, farmland);
    }
  }
}

fn tool_deactivate(
  tool: Res<Tool>,
  interface: Res<Interface>,
  mouse: Res<ButtonInput<MouseButton>>,
  world_index: Res<WorldIndex>,
  mut commands: Commands,
) {
  if mouse.pressed(MouseButton::Right) {
    if let Some(commands) = interface
      .selected_grass(&world_index)
      .map(|selected_grass| commands.entity(selected_grass))
    {
      tool.deactivate(commands);
    }
  }
}

fn tool_cycle(
  mut tool: ResMut<Tool>,
  server: Res<AssetServer>,
  mut icon: Query<&mut UiImage, With<ToolIcon>>,
  kbd: Res<ButtonInput<KeyCode>>,
) {
  if kbd.just_pressed(KeyCode::Tab) {
    if kbd.pressed(KeyCode::ShiftLeft) {
      tool.rev_cycle();
    } else {
      tool.cycle();
    }
    let texture = tool.texture(&server);
    icon
      .par_iter_mut()
      .for_each(move |mut ui_img| ui_img.texture = texture.clone());
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
        (
          update_cursor,
          update_arability,
          update_selector,
          tool_activate,
          tool_deactivate,
          tool_cycle,
        ),
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
  pub fn selected_grass(&self, world_index: &WorldIndex) -> Option<Entity> {
    let curs_coords = self.cursor_grid_coords();
    world_index
      .get(curs_coords)
      .filter(|(_, typ)| *typ == TileType::Grass)
      .map(|(ent, _)| ent)
  }
}
