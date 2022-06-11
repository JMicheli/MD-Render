mod mdr_application;
mod mdr_core;
mod mdr_scene;

use mdr_application::MdrApplication;
use mdr_core::{MdrEngine, MdrEngineOptions};
use winit::event_loop::EventLoop;

fn main() {
  println!("Starting MD Renderer");

  // Create event loop
  let event_loop = EventLoop::new();
  // Create engine
  let engine = MdrEngine::new(
    &event_loop,
    MdrEngineOptions {
      name: "MD Renderer Test".to_string(),
      debug: true,
    },
  );
  // Create application
  let mut app = MdrApplication::new(engine);

  // Start event loop
  event_loop.run(
    move |event, _, control_flow| match app.handle_event(&event) {
      Some(flow) => *control_flow = flow,
      None => (),
    },
  );
}
