use log::trace;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode};

pub struct MdrInputState {
  pub left: bool,
  pub up: bool,
  pub right: bool,
  pub down: bool,

  pub w: bool,
  pub a: bool,
  pub s: bool,
  pub d: bool,
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

        w: false,
        a: false,
        s: false,
        d: false,
      },
    }
  }

  pub fn keyboard_input(&mut self, input: &KeyboardInput) {
    trace!("Keyboard event");
    if input.virtual_keycode.is_none() {
      return;
    }

    match input.state {
      ElementState::Pressed => match input.virtual_keycode.unwrap() {
        VirtualKeyCode::Left => self.state.left = true,
        VirtualKeyCode::Up => self.state.up = true,
        VirtualKeyCode::Right => self.state.right = true,
        VirtualKeyCode::Down => self.state.down = true,

        VirtualKeyCode::W => self.state.w = true,
        VirtualKeyCode::A => self.state.a = true,
        VirtualKeyCode::S => self.state.s = true,
        VirtualKeyCode::D => self.state.d = true,

        _ => (),
      },
      ElementState::Released => match input.virtual_keycode.unwrap() {
        VirtualKeyCode::Left => self.state.left = false,
        VirtualKeyCode::Up => self.state.up = false,
        VirtualKeyCode::Right => self.state.right = false,
        VirtualKeyCode::Down => self.state.down = false,

        VirtualKeyCode::W => self.state.w = false,
        VirtualKeyCode::A => self.state.a = false,
        VirtualKeyCode::S => self.state.s = false,
        VirtualKeyCode::D => self.state.d = false,

        _ => (),
      },
    }
  }

  pub fn mouse_input(&mut self, _state: &ElementState, _button: &MouseButton) {
    trace!("Mouse event");
  }
}
