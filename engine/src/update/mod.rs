use std::time::Instant;

use crate::{input::MdrInputState, scene::MdrScene};

const CAMERA_ROT_SPEED: f32 = 5.0;

pub struct MdrUpdateContext {
  last_instant: Instant,
}

impl MdrUpdateContext {
  pub fn new() -> Self {
    Self {
      last_instant: Instant::now(),
    }
  }

  pub fn update_scene(&mut self, scene: &mut MdrScene, input_state: &MdrInputState) {
    let current_instant = Instant::now();
    let dt = (current_instant - self.last_instant).as_secs_f32();

    if input_state.left {
      scene.camera.rotate(dt * CAMERA_ROT_SPEED);
    } else if input_state.right {
      scene.camera.rotate(dt * -CAMERA_ROT_SPEED);
    }

    self.last_instant = current_instant;
  }
}
