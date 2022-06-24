use log::trace;
use winit::event::{ElementState, KeyboardInput, MouseButton};

mod key_state;
mod mouse_state;

pub struct MdrInputContext {}

impl MdrInputContext {
  pub fn new() -> Self {
    Self {}
  }

  pub fn keyboard_input(&mut self, input: &KeyboardInput) {
    trace!("Keyboard event");
  }

  pub fn mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
    trace!("Mouse event");
  }
}
