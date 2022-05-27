mod mdr_device;
mod mdr_engine;
mod mdr_pipeline;
mod mdr_swapchain;
mod mdr_window;

fn main() {
  println!("Starting test");

  let engine_name = Some("MD Renderer Test");
  let engine = mdr_engine::MdrEngine::new(engine_name);

  engine.run();

  println!("Exiting test");
}
