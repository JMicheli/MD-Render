mod logger;

use cgmath::Vector3;
use log::info;

use mdr_engine::{MdrEngine, MdrEngineOptions, MdrSceneObject};

fn main() {
  logger::init().expect("Failed to initialize logger");

  let opts = MdrEngineOptions { debug: false };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  // Ground plane
  let mut ground_plane = MdrSceneObject::from_obj("example/src/assets/plane.obj");
  ground_plane.transform.position = Vector3::new(0.0, 1.0, 0.0);
  engine.scene.add_object(ground_plane);
  // Sphere
  let mut sphere = MdrSceneObject::from_obj("example/src/assets/sphere.obj");
  sphere.transform.position = Vector3::new(2.0, -2.0, -1.0);
  engine.scene.add_object(sphere);
  // Suzanne
  let monkey = MdrSceneObject::from_obj("example/src/assets/suzanne.obj");
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
