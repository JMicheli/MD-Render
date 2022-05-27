use std::sync::Arc;

use vulkano::{instance::Instance, pipeline::graphics::viewport::Viewport, swapchain::Surface};
use vulkano_win::VkSurfaceBuild;

use winit::{
  dpi::{LogicalSize, PhysicalSize},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

pub struct MdrWindowOptions<'a> {
  pub width: u32,
  pub height: u32,
  pub title: &'a str,
}

pub struct MdrWindow {
  pub surface: Arc<Surface<Window>>,
}

impl MdrWindow {
  pub fn new(
    instance: &Arc<Instance>,
    event_loop: &EventLoop<()>,
    options: &MdrWindowOptions,
  ) -> Self {
    // Set up event loop and build window
    let surface = WindowBuilder::new()
      .with_title(options.title)
      .with_inner_size(LogicalSize::new(
        f64::from(options.width),
        f64::from(options.height),
      ))
      .with_resizable(false)
      .build_vk_surface(event_loop, instance.clone())
      .unwrap();

    return Self { surface };
  }

  pub fn create_viewport(&self) -> Viewport {
    return Viewport {
      origin: [0.0, 0.0],
      dimensions: self.dimensions().into(),
      depth_range: 0.0..1.0,
    };
  }

  pub fn dimensions(&self) -> PhysicalSize<u32> {
    return self.surface.window().inner_size();
  }
}
