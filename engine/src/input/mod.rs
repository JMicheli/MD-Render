use log::trace;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode};

pub struct MdrInputState {
  pub left: bool,
  pub up: bool,
  pub right: bool,
  pub down: bool,
}

pub struct MdrInputContext {
  pub state: MdrInputState,
}

impl MdrInputContext {
  pub fn new() -> Self {
    Self {
      state: MdrInputState {
        left: false,
        up: false,
        right: false,
        down: false,
      },
    }
  }

  pub fn keyboard_input(&mut self, input: &KeyboardInput) {
    trace!("Keyboard event");
    match input.state {
      ElementState::Pressed => match input.virtual_keycode.unwrap() {
        VirtualKeyCode::Left => self.state.left = true,
        VirtualKeyCode::Up => self.state.up = true,
        VirtualKeyCode::Right => self.state.right = true,
        VirtualKeyCode::Down => self.state.down = true,
        _ => (),
      },
      ElementState::Released => match input.virtual_keycode.unwrap() {
        VirtualKeyCode::Left => self.state.left = false,
        VirtualKeyCode::Up => self.state.up = false,
        VirtualKeyCode::Right => self.state.right = false,
        VirtualKeyCode::Down => self.state.down = false,
        _ => (),
      },
    }
  }

  pub fn mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
    trace!("Mouse event");
  }
}
