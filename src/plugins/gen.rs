use bevy::{
  app::{App, Plugin},
  prelude::Resource,
};
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, RngCore};

#[derive(Resource)]
pub struct WorldGen {
  perlin: Perlin,
}

impl WorldGen {
  pub fn new() -> Self {
    let mut rng = thread_rng();
    Self {
      perlin: Perlin::new(rng.next_u32()),
    }
  }

  pub fn at(&self, x: i32, y: i32) -> f32 {
    (self.perlin.get([x as f64 / 256., y as f64 / 256.]) as f32)
      .tanh()
      .abs()
  }
}

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(WorldGen::new());
  }
}
