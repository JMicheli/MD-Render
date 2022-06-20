use log::{info, trace};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

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

    (engine, event_loop)
  }

  pub fn handle_event(&mut self, event: Event<()>) -> Option<ControlFlow> {
    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => {
        info!("Exiting");
        Some(ControlFlow::Exit)
      }
      Event::WindowEvent {
        event: WindowEvent::Resized(_),
        ..
      } => {
        trace!("Resized");
        self.graphics_context.notify_resized();
        None
      }
      Event::MainEventsCleared => {
        self.graphics_context.draw();
        None
      }
      _ => None,
    }
  }
}
