use std::sync::Arc;

use winit::{
  event::{Event, WindowEvent},
  event_loop::ControlFlow,
};

use crate::{
  mdr_core::MdrEngine,
  mdr_scene::{mdr_camera::MdrCamera, MdrScene},
};

pub struct MdrApplication {
  engine: Arc<MdrEngine>,
  scene: Arc<MdrScene>,
}

impl MdrApplication {
  pub fn new(engine: Arc<MdrEngine>) -> Self {
    // Set up scene
    let scene = MdrScene::new(&engine);

    Self { engine, scene }
  }

  pub fn handle_event(&mut self, event: &Event<()>) -> Option<ControlFlow> {
    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => return Some(ControlFlow::Exit),
      Event::WindowEvent {
        event: WindowEvent::Resized(_),
        ..
      } => {
        self.engine.get_window().was_resized = true;
        None
      }
      Event::MainEventsCleared => {
        self.engine.render(self.scene.clone());
        None
      }
      _ => None,
    }
  }
}
