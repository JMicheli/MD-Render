use log::{info, trace};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

use crate::{
  context::MdrGraphicsContext, input::MdrInputContext, scene::MdrScene, update::MdrUpdateContext,
};

pub struct MdrEngineOptions {
  pub debug: bool,
}

pub struct MdrEngine {
  pub scene: MdrScene,

  graphics_context: MdrGraphicsContext,
  input_context: MdrInputContext,
  update_context: MdrUpdateContext,
}

impl MdrEngine {
  pub fn new(options: MdrEngineOptions) -> (Self, EventLoop<()>) {
    let event_loop = EventLoop::new();

    let engine = Self {
      scene: MdrScene::new(),

      graphics_context: MdrGraphicsContext::new(&event_loop, options.debug),
      input_context: MdrInputContext::new(),
      update_context: MdrUpdateContext::new(),
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
      Event::MainEventsCleared => {
        self
          .update_context
          .update_scene(&mut self.scene, &self.input_context.state);
        None
      }
      Event::RedrawEventsCleared => {
        self
          .graphics_context
          .update_scene_aspect_ratio(&mut self.scene);
        self.graphics_context.draw(&self.scene);
        None
      }
      _ => None,
    }
  }
}
