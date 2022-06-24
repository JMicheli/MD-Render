use log::{info, trace};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

use crate::{context::MdrGraphicsContext, input::MdrInputContext, scene::MdrScene};

pub struct MdrEngineOptions {
  pub debug: bool,
}

pub struct MdrEngine {
  pub scene: MdrScene,

  graphics_context: MdrGraphicsContext,
  input_context: MdrInputContext,
}

impl MdrEngine {
  pub fn new(options: MdrEngineOptions) -> (Self, EventLoop<()>) {
    let event_loop = EventLoop::new();

    let engine = Self {
      scene: MdrScene::new(),

      graphics_context: MdrGraphicsContext::new(&event_loop, options.debug),
      input_context: MdrInputContext::new(),
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
      Event::WindowEvent {
        event: WindowEvent::MouseInput { state, button, .. },
        ..
      } => {
        self.input_context.mouse_input(&state, &button);
        None
      }
      Event::WindowEvent {
        event: WindowEvent::KeyboardInput { input, .. },
        ..
      } => {
        self.input_context.keyboard_input(&input);
        None
      }
      Event::MainEventsCleared => None,
      Event::RedrawRequested(_) => {
        self.graphics_context.draw(&self.scene);
        None
      }
      _ => None,
    }
  }
}
