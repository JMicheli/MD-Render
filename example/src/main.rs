mod logger;

use cgmath::Vector3;
use log::info;

use mdr_engine::{MdrEngine, MdrEngineOptions, MdrSceneObject};

fn main() {
  logger::init().expect("Failed to initialize logger");

  let opts = MdrEngineOptions { debug: true };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  let mut monkey = MdrSceneObject::from_obj("example/src/assets/suzanne.obj");
  monkey.transform.position = Vector3::new(0.0, 1.0, 0.0);
  engine.scene.add_object(monkey);

  //for i in -5..=5 {
  //  for j in -5..=5 {
  //    let mut triangle = MdrSceneObject::test_triangle();
  //    triangle.transform.position = Vector3::new(i as f32, j as f32, 0.0);
  //
  //    engine.scene.add_object(triangle);
  //  }
  //}

  // Start event loop
  info!("Starting event loop");
  event_loop.run(
    move |event, _, control_flow| match engine.handle_event(event) {
      Some(flow) => *control_flow = flow,
      None => (),
    },
  );
}
