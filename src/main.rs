mod mdr_application;
mod mdr_device;
mod mdr_pipeline;
mod mdr_swapchain;
mod mdr_window;

fn main() {
  println!("Starting test");

  let app = mdr_application::MdrApp::new("MD Renderer Test");

  app.run();

  println!("Exiting test");
}
