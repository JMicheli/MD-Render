mod logger;

use log::info;

use mdr_engine::{MdrEngine, MdrEngineOptions};

fn main() {
  logger::init().expect("Failed to initialize logger");

  let opts = MdrEngineOptions { debug: false };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  // Start event loop
  info!("Starting event loop");
  event_loop.run(
    move |event, _, control_flow| match engine.handle_event(event) {
      Some(flow) => *control_flow = flow,
      None => (),
    },
  );
}
