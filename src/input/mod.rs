use log::trace;
use winit::{
  dpi::PhysicalPosition,
  event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode},
};

pub struct MdrInputState {
  pub left: bool,
  pub up: bool,
  pub right: bool,
  pub down: bool,

  pub w: bool,
  pub a: bool,
  pub s: bool,
  pub d: bool,

  pub mouse_position: [f32; 2],
  pub mouse_left: bool,
  pub mouse_right: bool,
  pub mouse_delta: [f32; 2],
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

        mouse_position: [0.0, 0.0],
        mouse_left: false,
        mouse_right: false,
        mouse_delta: [0.0, 0.0],
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

  pub fn mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
    trace!("Mouse event");
    match (button, state) {
      (MouseButton::Left, ElementState::Pressed) => {
        self.state.mouse_left = true;
      }
      (MouseButton::Right, ElementState::Pressed) => {
        self.state.mouse_right = true;
      }
      (MouseButton::Left, ElementState::Released) => {
        self.state.mouse_left = false;
      }
      (MouseButton::Right, ElementState::Released) => {
        self.state.mouse_right = false;
      }
      _ => {}
    }
  }

  pub fn mouse_moved_input(&mut self, position: PhysicalPosition<f64>) {
    trace!("Mouse moved event");
    let new_position = [position.x as f32, position.y as f32];
    self.state.mouse_delta = [
      new_position[0] - self.state.mouse_position[0],
      new_position[1] - self.state.mouse_position[1],
    ];
    self.state.mouse_position = new_position;
  }

  pub fn cleanup_after_update(&mut self) {
    // Zero mouse delta in case mouse has stopped moving
    self.state.mouse_delta = [0.0, 0.0];
  }
}
