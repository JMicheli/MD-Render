mod mdr_device;
mod mdr_engine;
mod mdr_pipeline;
mod mdr_swapchain;
mod mdr_window;

// Consts
const DEBUG_ENABLED: bool = true;

fn main() {
  println!("Starting MD Renderer");

  let engine_name = Some("MD Renderer Test");
  let engine = mdr_engine::MdrEngine::new(DEBUG_ENABLED, engine_name);

  engine.run();

  println!("Exiting main");
}
