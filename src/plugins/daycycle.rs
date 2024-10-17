use bevy::{
  app::{App, Plugin, Update},
  log::info,
  prelude::{Event, EventWriter, Res, ResMut, Resource},
  time::Time,
};
use std::time::Duration;

const DAY_LEN_SEC: f32 = 240.;

#[derive(Event)]
pub struct NewDayEvent;

#[derive(PartialEq)]
pub enum TimeMode {
  Day,
  Pending,
  Night,
}

// 0.0: 7 AM
// 0.5: 7 PM
// 1.0: 7 AM (next day)

#[derive(Resource)]
pub struct DayCycle {
  pub daytime: f32,
  pub mode: TimeMode,
  pub day: usize,
}

fn tick(
  time: Res<Time>,
  mut day: ResMut<DayCycle>,
  mut ev_newday: EventWriter<NewDayEvent>,
) {
  if day.tick(time.delta()) {
    ev_newday.send(NewDayEvent);
  }
}

impl DayCycle {
  fn tick(&mut self, delta_rt: Duration) -> bool {
    let rate: f32 = match self.mode {
      TimeMode::Day => 1.,
      TimeMode::Pending => 0.,
      TimeMode::Night => 24.,
    };

    let delta_gt = rate * delta_rt.as_secs_f32() / DAY_LEN_SEC;
    self.daytime += delta_gt;

    if self.mode == TimeMode::Day && self.daytime > 0.5 {
      info!("Switch to pending mode");
      self.daytime = 0.5;
      self.mode = TimeMode::Pending;
    }

    if self.mode == TimeMode::Night && self.daytime > 1. {
      info!("Daytime again");
      self.daytime = 0.;
      self.mode = TimeMode::Day;
      self.day += 1;
      return true;
    }
    false
  }
  pub fn sleep(&mut self) {
    info!("Sleeping");
    self.mode = TimeMode::Night;
  }
}

pub struct DayCyclePlugin;

impl Plugin for DayCyclePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(DayCycle {
        daytime: 0.,
        mode: TimeMode::Day,
        day: 1,
      })
      .add_systems(Update, tick)
      .add_event::<NewDayEvent>();
  }
}
