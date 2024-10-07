use bevy::{
  app::{Plugin, Startup, Update},
  asset::AssetServer,
  color::palettes::css::{BLACK, RED},
  math::Vec3,
  prelude::{
    default, Commands, Component, Query, Res, Resource, TextBundle, Transform,
    Visibility, With,
  },
  sprite::SpriteBundle,
  text::{Text, TextSection, TextStyle},
};

#[derive(Resource)]
pub struct Hud {
  pub arability: Option<f32>,
  pub selector_pos: Option<Vec3>,
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
      visibility: Visibility::Hidden,
      ..default()
    },
  ));
}

fn update_arability(
  mut texts: Query<&mut Text, With<ArabilityText>>,
  hud: Res<Hud>,
) {
  for mut text in &mut texts {
    text.sections[1].value = if let Some(arability) = hud.arability {
      format!("{arability:.2}")
    } else {
      String::from("Unavailable")
    };
  }
}

fn update_selector(
  mut selector: Query<(&mut Visibility, &mut Transform), With<Selector>>,
  hud: Res<Hud>,
) {
  let (mut visibility, mut transform) = selector.single_mut();
  if let Some(pos) = hud.selector_pos {
    transform.translation.x = pos.x;
    transform.translation.y = pos.y;
    transform.translation.z = 100.;
    *visibility = Visibility::Inherited;
  } else {
    *visibility = Visibility::Hidden;
  }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .insert_resource(Hud {
        arability: None,
        selector_pos: None,
      })
      .add_systems(Startup, setup)
      .add_systems(Update, (update_arability, update_selector));
  }
}
