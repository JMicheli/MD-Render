use std::time::Instant;

use crate::{input::MdrInputState, scene::MdrScene};

const CAMERA_ROT_SPEED: f32 = 2.5;
const CAMERA_MOV_SPEED: f32 = 1.0;

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

    // Rotate right
    if input_state.right {
      scene.camera.theta += dt * CAMERA_ROT_SPEED;
    }
    // Rotate left
    if input_state.left {
      scene.camera.theta += dt * -CAMERA_ROT_SPEED;
    }
    // Rotate up
    if input_state.up {
      scene.camera.phi += dt * -CAMERA_ROT_SPEED;
    }
    // Rotate down
    if input_state.down {
      scene.camera.phi += dt * CAMERA_ROT_SPEED;
    }

    self.last_instant = current_instant;
  }
}
