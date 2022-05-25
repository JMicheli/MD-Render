use std::sync::Arc;

use vulkano::{
  format::Format,
  instance::Instance,
  pipeline::graphics::viewport::Viewport,
  render_pass::{Framebuffer, RenderPass},
  swapchain::Surface,
};
use vulkano_win::VkSurfaceBuild;

use winit::{
  dpi::{LogicalSize, PhysicalSize},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

use crate::{mdr_device::MdrDevice, mdr_swapchain::MdrSwapchain};

pub struct MdrWindowOptions<'a> {
  pub width: u32,
  pub height: u32,
  pub title: &'a str,
}

pub struct MdrWindow {
  pub event_loop: EventLoop<()>,
  surface: Arc<Surface<Window>>,
  swapchain: Option<MdrSwapchain>,
}

impl MdrWindow {
  pub fn new(instance: Arc<Instance>, options: MdrWindowOptions) -> Self {
    // Set up event loop and build window
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
      .with_title(options.title)
      .with_inner_size(LogicalSize::new(
        f64::from(options.width),
        f64::from(options.height),
      ))
      .with_resizable(false)
      .build_vk_surface(&event_loop, instance.clone())
      .unwrap();

    return Self {
      event_loop,
      surface,
      swapchain: None,
    };
  }

  pub fn initialize_swapchain(&mut self, device: Arc<MdrDevice>) {
    let initialized_swapchain = Some(MdrSwapchain::new(device, self.surface.clone()));
    self.swapchain = initialized_swapchain;
  }

  pub fn create_viewport(&self) -> Viewport {
    return Viewport {
      origin: [0.0, 0.0],
      dimensions: self.dimensions().into(),
      depth_range: 0.0..1.0,
    };
  }

  pub fn get_render_pass(&self) -> Arc<RenderPass> {
    let swapchain = self
      .swapchain
      .as_ref()
      .expect("Tried to get render pass before initializing swapchain.");

    return swapchain.create_render_pass();
  }

  pub fn get_frame_buffers(&self, render_pass: Arc<RenderPass>) -> Vec<Arc<Framebuffer>> {
    let swapchain = self
      .swapchain
      .as_ref()
      .expect("Tried to get framebuffers before initializing swapchain.");
    return swapchain.create_frame_buffers(render_pass);
  }

  pub fn surface(&self) -> Arc<Surface<Window>> {
    return self.surface.clone();
  }

  pub fn swapchain_image_format(&self) -> Format {
    return self
      .swapchain
      .as_ref()
      .expect("Attempted to access swapchain before initialization")
      .image_format();
  }

  pub fn regenerate_swapchain(&mut self) {
    let mut swapchain = self
      .swapchain
      .as_mut()
      .expect("Attempted to access swapchain before initialization");
    swapchain.regenerate();
  }

  pub fn dimensions(&self) -> PhysicalSize<u32> {
    return self.surface.window().inner_size();
  }
}
