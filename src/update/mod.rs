use std::time::Instant;

use crate::{input::MdrInputState, scene::MdrScene};

const CAMERA_MOV_SPEED: f32 = 0.5;
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

    if input_state.w {
      scene.camera.transform.translation.z += dt * CAMERA_MOV_SPEED;
    }
    if input_state.a {
      scene.camera.transform.translation.x += dt * CAMERA_MOV_SPEED;
    }
    if input_state.d {
      scene.camera.transform.translation.x += dt * -CAMERA_MOV_SPEED;
    }
    if input_state.s {
      scene.camera.transform.translation.z += dt * -CAMERA_MOV_SPEED;
    }

    if input_state.up {
      scene.camera.transform.rotation.x += dt * CAMERA_ROT_SPEED;
    }
    if input_state.down {
      scene.camera.transform.rotation.x += dt * -CAMERA_ROT_SPEED;
    }
    if input_state.right {
      scene.camera.transform.rotation.z += dt * CAMERA_ROT_SPEED;
    }
    if input_state.left {
      scene.camera.transform.rotation.z += dt * -CAMERA_ROT_SPEED;
    }

    self.last_instant = current_instant;
  }
}
