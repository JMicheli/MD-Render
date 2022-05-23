use winit::{
  dpi::PhysicalSize,
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

pub struct MdrWindow {
  pub event_loop: EventLoop<()>,
  window: Window,

  should_close: bool,
}

impl MdrWindow {
  pub fn new(width: u32, height: u32, title: &str) -> Self {
    let window_builder = WindowBuilder::new()
      .with_title(title)
      .with_inner_size(PhysicalSize::new(width, height))
      .with_resizable(false);

    // Set up event loop and build window
    let event_loop = EventLoop::new();
    let window = window_builder.build(&event_loop).unwrap();

    Self {
      window,
      event_loop,
      should_close: false,
    }
  }
}
