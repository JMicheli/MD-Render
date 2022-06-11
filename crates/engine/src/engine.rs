use winit::event_loop::EventLoop;

use crate::graphics_context::MdrGraphicsContext;

pub struct MdrEngineOptions {
  pub debug: bool,
}

pub struct MdrEngine {
  graphics_context: MdrGraphicsContext,
}

impl MdrEngine {
  pub fn new(options: MdrEngineOptions) -> (Self, EventLoop<()>) {
    // Create event loop
    let event_loop = EventLoop::new();
    let graphics_context = MdrGraphicsContext::new(&event_loop, options.debug);

    let engine = Self { graphics_context };

    return (engine, event_loop);
  }
}
