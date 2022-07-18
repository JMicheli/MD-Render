use std::env;

use log::info;

use mdr_engine::resources::texture::{MdrSamplerMode, MdrTextureCreateInfo};
use mdr_engine::resources::{MdrColorType, MdrMaterialCreateInfo, MdrRgb};
use mdr_engine::{
  logger,
  scene::{MdrLight, MdrRenderObject},
  MdrEngine, MdrEngineOptions,
};

// Some functions and constants extraneous to the example
mod utils;
use utils::{asset, DEBUG_ENABLED, MDR_LOG_LEVEL};

// Consts for this example
const LIGHT_MOV_SPEED: f32 = 1.0;
const LIGHT_BRIGHTNESS: f32 = 2.0;
const CAMERA_MOV_SPEED: f32 = 0.5;
const CAMERA_ROT_SPEED: f32 = 0.01;

fn main() {
  env::set_var("MDR_LOG_LEVEL", MDR_LOG_LEVEL);
  logger::init_from_env().expect("Failed to initialize logger");

  let opts = MdrEngineOptions {
    debug: DEBUG_ENABLED,
  };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  // Create object meshes
  let monkey_mesh = engine
    .manage_resources()
    .load_mesh_obj(asset("meshes/suzanne.obj").as_str(), "monkey")
    .unwrap();
  let sphere_mesh = engine
    .manage_resources()
    .load_mesh_obj(asset("meshes/sphere.obj").as_str(), "sphere")
    .unwrap();
  let cube_mesh = engine
    .manage_resources()
    .load_mesh_obj(asset("meshes/cube.obj").as_str(), "cube")
    .unwrap();
  let plane_mesh = engine
    .manage_resources()
    .load_mesh_obj(asset("meshes/plane.obj").as_str(), "plane")
    .unwrap();

  // Create textures
  let metal_plates_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/metal_plates/base_color.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "metal_plates_base_color",
    )
    .unwrap();
  let metal_plates_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/metal_plates/roughness.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "metal_plates_roughness",
    )
    .unwrap();
  let blue_tiles_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/blue_tiles/base_color.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "blue_tiles_base_color",
    )
    .unwrap();
  let blue_tiles_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/blue_tiles/roughness.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "blue_tiles_roughness",
    )
    .unwrap();
  let wood_planks_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/wood_planks/base_color.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "wood_planks_base_color",
    )
    .unwrap();
  let wood_planks_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/wood_planks/roughness.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "wood_planks_roughness",
    )
    .unwrap();
  let red_fabric_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/red_fabric/base_color.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "red_fabric_base_color",
    )
    .unwrap();
  let red_fabric_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: asset("textures/red_fabric/roughness.png").as_str(),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "red_fabric_roughness",
    )
    .unwrap();

  // Create object materials
  let monkey_mat = engine
    .manage_resources()
    .create_material(
      MdrMaterialCreateInfo {
        diffuse: red_fabric_base_color,
        roughness: red_fabric_roughness,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      "monkey_mat",
    )
    .unwrap();
  let sphere_mat = engine
    .manage_resources()
    .create_material(
      MdrMaterialCreateInfo {
        diffuse: blue_tiles_base_color,
        roughness: blue_tiles_roughness,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      "sphere_mat",
    )
    .unwrap();
  let cube_mat = engine
    .manage_resources()
    .create_material(
      MdrMaterialCreateInfo {
        diffuse: metal_plates_base_color,
        roughness: metal_plates_roughness,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      "cube_mat",
    )
    .unwrap();
  let plane_mat = engine
    .manage_resources()
    .create_material(
      MdrMaterialCreateInfo {
        diffuse: wood_planks_base_color,
        roughness: wood_planks_roughness,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      "plane_mat",
    )
    .unwrap();

  // Add suzanne
  let mut monkey = MdrRenderObject::new(monkey_mesh, monkey_mat);
  monkey.transform.translation.set(0.0, 0.0, -2.0);
  engine.scene.add_object(monkey);
  // Add sphere
  let mut sphere = MdrRenderObject::new(sphere_mesh, sphere_mat);
  sphere.transform.translation.set(2.0, -2.0, -3.0);
  engine.scene.add_object(sphere);
  // Add cube
  let mut cube = MdrRenderObject::new(cube_mesh, cube_mat);
  cube.transform.translation.set(-2.0, -2.0, -3.0);
  engine.scene.add_object(cube);
  // Add ground plane
  let mut ground_plane = MdrRenderObject::new(plane_mesh, plane_mat);
  ground_plane.transform.translation.set(0.0, 1.0, 0.0);
  engine.scene.add_object(ground_plane);

  // Add white light
  let mut white_light = MdrLight::white(LIGHT_BRIGHTNESS);
  white_light.translation.set(1.0, 3.0, 3.0);
  engine.scene.lights.add_light(white_light);

  // Set update function
  engine.set_update_function(Box::new(|scene, input_state, dt| {
    // Camera WASD movement
    if input_state.w {
      let move_magnitude = dt * CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_forward_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.d {
      let move_magnitude = dt * -CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_sideways_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.a {
      let move_magnitude = dt * CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_sideways_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.s {
      let move_magnitude = dt * -CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_forward_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }

    // Light movement with arrow keys
    if scene.lights.get_count() > 0 {
      let light = scene.lights.get_light_mut(0).unwrap();

      if input_state.up {
        light.translation.z += dt * LIGHT_MOV_SPEED;
      }
      if input_state.down {
        light.translation.z += dt * -LIGHT_MOV_SPEED;
      }
      if input_state.right {
        light.translation.x += dt * LIGHT_MOV_SPEED;
      }
      if input_state.left {
        light.translation.x += dt * -LIGHT_MOV_SPEED;
      }
    }

    // Camera rotation with mouse when right-button pressed
    if input_state.mouse_right {
      let delta_x = input_state.mouse_delta[0];
      let delta_y = input_state.mouse_delta[1];

      scene.camera.transform.rotation.z += delta_x * CAMERA_ROT_SPEED;
      scene.camera.transform.rotation.x += delta_y * -CAMERA_ROT_SPEED;
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
