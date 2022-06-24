use std::sync::Arc;

use log::{info, trace};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

use crate::{graphics_context::MdrGraphicsContext, scene::MdrScene};

pub struct MdrEngineOptions {
  pub debug: bool,
}

pub struct MdrEngine {
  pub scene: MdrScene,

  graphics_context: MdrGraphicsContext,
}

impl MdrEngine {
  pub fn new(options: MdrEngineOptions) -> (Self, EventLoop<()>) {
    let event_loop = EventLoop::new();

    let engine = Self {
      graphics_context: MdrGraphicsContext::new(&event_loop, options.debug),
      scene: MdrScene::new(),
    };

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
        self.graphics_context.draw(&self.scene);
        None
      }
      _ => None,
    }
  }
}
