use std::time::Instant;

use crate::{input::MdrInputState, scene::MdrScene};

const CAMERA_ROT_SPEED: f32 = 2.5;

pub struct MdrUpdateContext {
  last_instant: Instant,
}

impl MdrUpdateContext {
  pub fn new() -> Self {
    Self {
      last_instant: Instant::now(),
    }
  }

  // TODO Externalize
  pub fn update_scene(&mut self, scene: &mut MdrScene, input_state: &MdrInputState) {
    let current_instant = Instant::now();
    let dt = (current_instant - self.last_instant).as_secs_f32();

    self.last_instant = current_instant;
  }
}
