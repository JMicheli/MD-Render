mod mdr_window;

use winit::{
  event::{Event, WindowEvent},
  event_loop::ControlFlow,
};

use mdr_window::MdrWindow;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
  println!("Starting test");

  let window = MdrWindow::new(WIDTH, HEIGHT, "MD Renderer Test");

  window.event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        window_id: _,
        event: WindowEvent::CloseRequested,
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });

  println!("Exiting main");
}
