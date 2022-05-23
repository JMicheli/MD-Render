mod mdr_window;

use mdr_window::MdrWindow;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
  println!("Starting test");

  let window = MdrWindow::new(WIDTH, HEIGHT, "MD Renderer Test");

  println!("Exiting main");
}
