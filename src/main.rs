mod mdr_core;
mod mdr_scene;

use mdr_core::MdrEngine;

// Consts
const DEBUG_ENABLED: bool = true;

fn main() {
  println!("Starting MD Renderer");

  let engine_name = Some("MD Renderer Test");
  let engine = MdrEngine::new(DEBUG_ENABLED, engine_name);

  engine.run();

  println!("Exiting main");
}
