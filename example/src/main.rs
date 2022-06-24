mod logger;

use log::info;

use mdr_engine::{MdrEngine, MdrEngineOptions, MdrSceneObject};

fn main() {
  logger::init().expect("Failed to initialize logger");

  let opts = MdrEngineOptions { debug: true };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  let triangle = MdrSceneObject::test_triangle();
  let monkey = MdrSceneObject::from_obj("example/src/assets/suzanne.obj");
  engine.scene.add_object(triangle);
  engine.scene.add_object(monkey);

  // Start event loop
  info!("Starting event loop");
  event_loop.run(
    move |event, _, control_flow| match engine.handle_event(event) {
      Some(flow) => *control_flow = flow,
      None => (),
    },
  );
}
