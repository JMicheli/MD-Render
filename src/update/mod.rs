use std::time::Instant;

use crate::{input::MdrInputState, scene::MdrScene};

pub struct MdrUpdateContext {
  update_function: Box<dyn FnMut(&mut MdrScene, &MdrInputState, f32) -> ()>,

  last_instant: Instant,
}

impl MdrUpdateContext {
  pub fn new() -> Self {
    Self {
      last_instant: Instant::now(),
      update_function: Box::new(|_, _, _| {}),
    }
  }

  /// Set the update function to use each frame, the function should be in the form
  /// of a closure which operates on the following:
  ///   * `&mut MdrScene` - A mutable reference to the scene being updated.
  ///   * `&MdrInputState` - A reference to the input state this frame.
  ///   * `f32` - the time delta since last frame in seconds.
  pub fn set_update_function(
    &mut self,
    f: Box<dyn FnMut(&mut MdrScene, &MdrInputState, f32) -> ()>,
  ) {
    self.update_function = f;
  }

  /// Calculates time since last frame and performs updates to the scene according to the
  /// user function set with `set_update_function()`.
  pub(crate) fn update_scene(&mut self, scene: &mut MdrScene, input_state: &MdrInputState) {
    let current_instant = Instant::now();
    let dt = (current_instant - self.last_instant).as_secs_f32();

    // User-set update function
    (self.update_function)(scene, input_state, dt);

    self.last_instant = current_instant;
  }
}
