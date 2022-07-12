use std::{env, path::Path};

use log::info;

use mdr_engine::{logger, MdrEngine, MdrEngineOptions, MdrMaterial, MdrSceneObject};

// Consts for this example
const CAMERA_MOV_SPEED: f32 = 0.5;
const CAMERA_ROT_SPEED: f32 = 2.5;

// Build debug configuration
#[cfg(debug_assertions)]
const MDR_LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
const MDR_LOG_LEVEL: &str = "info";
#[cfg(debug_assertions)]
const DEBUG_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_ENABLED: bool = false;

// Asset handling
#[cfg(debug_assertions)]
const ASSET_PREFIX: &str = "examples/basic/assets/";
#[cfg(not(debug_assertions))]
const ASSET_PREFIX: &str = "assets/";

fn asset(asset_path: &str) -> String {
  let asset_path_prefix = Path::new(ASSET_PREFIX);
  asset_path_prefix
    .join(Path::new(asset_path))
    .to_str()
    .unwrap()
    .to_string()
}

fn main() {
  env::set_var("MDR_LOG_LEVEL", MDR_LOG_LEVEL);
  logger::init_from_env().expect("Failed to initialize logger");

  let opts = MdrEngineOptions {
    debug: DEBUG_ENABLED,
  };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  // Suzanne
  let mut monkey = MdrSceneObject::from_obj(asset("meshes/suzanne.obj").as_str());
  monkey.transform.translation.set(0.0, 0.0, -2.0);
  monkey.material = MdrMaterial::red();
  engine.scene.add_object(monkey);
  // Sphere
  let mut sphere = MdrSceneObject::from_obj(asset("meshes/sphere.obj").as_str());
  sphere.transform.translation.set(2.0, -2.0, -3.0);
  sphere.material = MdrMaterial::green();
  engine.scene.add_object(sphere);
  // Cube
  let mut cube = MdrSceneObject::from_obj(asset("meshes/cube.obj").as_str());
  cube.transform.translation.set(-2.0, -2.0, -3.0);
  cube.material = MdrMaterial::blue();
  engine.scene.add_object(cube);
  // Ground plane
  let mut ground_plane = MdrSceneObject::from_obj(asset("meshes/plane.obj").as_str());
  ground_plane.transform.translation.set(0.0, 1.0, 0.0);
  ground_plane.material = MdrMaterial::grey();
  engine.scene.add_object(ground_plane);

  // Set update function
  engine.set_update_function(Box::new(|scene, input_state, dt| {
    if input_state.w {
      scene.camera.transform.translation.z += dt * CAMERA_MOV_SPEED;
    }
    if input_state.a {
      scene.camera.transform.translation.x += dt * CAMERA_MOV_SPEED;
    }
    if input_state.d {
      scene.camera.transform.translation.x += dt * -CAMERA_MOV_SPEED;
    }
    if input_state.s {
      scene.camera.transform.translation.z += dt * -CAMERA_MOV_SPEED;
    }

    if input_state.up {
      scene.camera.transform.rotation.x += dt * CAMERA_ROT_SPEED;
    }
    if input_state.down {
      scene.camera.transform.rotation.x += dt * -CAMERA_ROT_SPEED;
    }
    if input_state.right {
      scene.camera.transform.rotation.z += dt * CAMERA_ROT_SPEED;
    }
    if input_state.left {
      scene.camera.transform.rotation.z += dt * -CAMERA_ROT_SPEED;
    }
  }));

  // Start event loop
  info!("Starting event loop");
  event_loop.run(
    move |event, _, control_flow| match engine.handle_event(event) {
      Some(flow) => *control_flow = flow,
      None => (),
    },
  );
}
